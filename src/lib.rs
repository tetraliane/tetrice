mod field;
mod checker;
mod tetrimino;

#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::VecDeque;

pub use field::Field;
pub use checker::Checker;
pub use tetrimino::{Shape, Tetrimino};

pub struct Game {
    field: Field,
    tetrimino: Tetrimino,
    queue: VecDeque<Tetrimino>,
    held: Option<Tetrimino>,
    selector: Box<dyn FnMut() -> Shape>,
    can_hold: bool,
    is_end: bool,
    removed_lines: usize,
}

impl Game {
    pub fn new(
        width: usize,
        height: usize,
        queue_size: usize,
        mut selector: Box<dyn FnMut() -> Shape>,
    ) -> Self {
        if width < 4 {
            panic!("not enough width")
        } else if height < 1 {
            panic!("not enough height")
        }

        let mut game = Game {
            field: Field::new(width, height),
            tetrimino: Tetrimino::new(selector()),
            queue: VecDeque::new(),
            held: None,
            selector,
            can_hold: true,
            is_end: false,
            removed_lines: 0,
        };
        game.init_pos();
        game.queue
            .resize_with(queue_size, || Tetrimino::new((game.selector)()));
        game
    }

    fn init_pos(&mut self) {
        let t = self.tetrimino.move_to((
            (self.field.width() - self.tetrimino.width()) as isize / 2,
            -1 * self.tetrimino.height() as isize,
        ));
        let lowest = (0..5)
            .map(|dist_up| t.move_up(dist_up))
            .find(|s| !Checker(&self.field, s).overlap());
        if let Some(l) = lowest {
            self.tetrimino = l;
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn tetrimino(&self) -> &Tetrimino {
        &self.tetrimino
    }

    pub fn queue(&self) -> &VecDeque<Tetrimino> {
        &self.queue
    }

    pub fn held(&self) -> Option<Tetrimino> {
        self.held.clone()
    }

    pub fn ghost(&self) -> Tetrimino {
        let bottom = self.tetrimino.bottom();
        let dist_down = self.field.height() as isize - bottom;
        (0..dist_down)
            .rev()
            .map(|dist_y| self.tetrimino.move_down(dist_y))
            .find(|t| {
                let check = Checker(&self.field, t);
                check.touch_down() && !check.overlap() && check.route_from(&self.tetrimino)
            })
            .unwrap()
    }

    pub fn check(&self) -> Checker {
        Checker(&self.field, &self.tetrimino)
    }

    pub fn is_end(&self) -> bool {
        return self.is_end;
    }

    pub fn removed_lines(&self) -> usize {
        self.removed_lines
    }

    pub fn move_left(&mut self) -> bool {
        if self.is_end {
            return false;
        }

        if !self.check().touch_left() {
            self.tetrimino = self.tetrimino.move_left(1);
            true
        } else {
            false
        }
    }
    pub fn move_right(&mut self) -> bool {
        if self.is_end {
            return false;
        }

        if !self.check().touch_right() {
            self.tetrimino = self.tetrimino.move_right(1);
            true
        } else {
            false
        }
    }
    pub fn soft_drop(&mut self) -> bool {
        if self.is_end {
            return false;
        }

        if !self.check().touch_down() {
            self.tetrimino = self.tetrimino.move_down(1);
            true
        } else {
            false
        }
    }

    pub fn rotate(&mut self) -> bool {
        if self.is_end {
            return false;
        }

        let new_tetrimino = self.tetrimino.rotate(1);
        let result = near_points()
            .iter()
            .map(|p| new_tetrimino.move_right(p.0).move_down(p.1))
            .find(|t| !Checker(&self.field, t).overlap());
        if let Some(t) = result {
            self.tetrimino = t;
            true
        } else {
            false
        }
    }

    pub fn hard_drop(&mut self) {
        if self.is_end {
            return;
        }

        self.tetrimino = self.ghost();
    }

    fn shift_queue(&mut self) -> Tetrimino {
        self.queue.push_back(Tetrimino::new((self.selector)()));
        self.queue.pop_front().unwrap()
    }

    pub fn save(&mut self) -> usize {
        if self.is_end {
            return 0;
        }

        for pos in self.tetrimino.blocks() {
            self.field.set(pos, self.tetrimino.color());
        }
        if self.tetrimino.bottom() < 0 {
            self.is_end = true;
        }
        self.tetrimino = self.shift_queue();
        self.init_pos();
        self.can_hold = true;

        let lines = self.field.remove_filled_lines();
        self.removed_lines += lines;
        lines
    }

    pub fn hold(&mut self) {
        if !self.can_hold || self.is_end {
            return;
        }

        let new_held = Tetrimino::new(self.tetrimino.shape()).move_to((0, 0));
        self.tetrimino = if let Some(current_held) = self.held.clone() {
            current_held
        } else {
            self.shift_queue()
        };
        self.held = Some(new_held);
        self.init_pos();
        self.can_hold = false;
    }
}

const DISTANCE_NEAR: isize = 2;

// Return points "near" the given vector, sorting them by pointIsPrior.
fn near_points() -> Vec<(isize, isize)> {
    let mut points: Vec<(isize, isize)> = (-DISTANCE_NEAR..=DISTANCE_NEAR)
        .flat_map(|x| (-DISTANCE_NEAR..=DISTANCE_NEAR).map(move |y| (x, y)))
        .collect();
    points.sort_by(point_is_prior);
    points
}

fn point_is_prior(point: &(isize, isize), other: &(isize, isize)) -> Ordering {
    let dist1 = point.0.pow(2) + point.1.pow(2);
    let dist2 = other.0.pow(2) + other.1.pow(2);
    if point == other {
        Ordering::Equal
    } else if dist1 == dist2 {
        if (point.1 == other.1 && point.0 > 0) || (point.1 != other.1 && point.1 > other.1) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    } else {
        if dist1 > dist2 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}
