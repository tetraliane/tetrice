mod field;
mod judge;
mod tetrimino;

#[cfg(test)]
mod tests;

use std::cmp::Ordering;

use field::Field;
use tetrimino::{Shape, Tetrimino};

pub struct Game {
    field: Field,
    tetrimino: Tetrimino,
    queue: Vec<Tetrimino>,
    selector: Box<dyn FnMut() -> Shape>,
    is_end: bool,
}

impl Game {
    pub fn new(
        width: usize,
        height: usize,
        queue_size: usize,
        mut selector: Box<dyn FnMut() -> Shape>,
    ) -> Self {
        let mut game = Game {
            field: Field::new(width, height),
            tetrimino: Tetrimino::new(selector()),
            queue: vec![],
            selector,
            is_end: false,
        };
        game.init_pos();
        game.queue
            .resize_with(queue_size, || Tetrimino::new((game.selector)()));
        game
    }

    fn init_pos(&mut self) {
        self.tetrimino = self.tetrimino.move_to((
            (self.field.width() - self.tetrimino.width()) as isize / 2,
            -1 * self.tetrimino.height() as isize,
        ))
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn tetrimino(&self) -> &Tetrimino {
        &self.tetrimino
    }

    pub fn queue(&self) -> &Vec<Tetrimino> {
        &self.queue
    }

    pub fn ghost(&self) -> Tetrimino {
        let bottom = self.tetrimino.bottom();
        let dist_down = self.field.height() as isize - bottom;
        (0..dist_down)
            .rev()
            .map(|dist_y| self.tetrimino.move_down(dist_y))
            .find(|t| {
                judge::touching_down(&self.field, t)
                    && !judge::overlapping(&self.field, t)
                    && judge::route_exists(&self.field, &self.tetrimino, t)
            })
            .unwrap()
    }

    pub fn is_end(&self) -> bool {
        return self.is_end;
    }

    pub fn move_left(&mut self) {
        if self.is_end {
            return;
        }

        if !judge::touching_left(&self.field, &self.tetrimino) {
            self.tetrimino = self.tetrimino.move_left(1);
        }
    }
    pub fn move_right(&mut self) {
        if self.is_end {
            return;
        }

        if !judge::touching_right(&self.field, &self.tetrimino) {
            self.tetrimino = self.tetrimino.move_right(1);
        }
    }
    pub fn soft_drop(&mut self) {
        if self.is_end {
            return;
        }

        if !judge::touching_down(&self.field, &self.tetrimino) {
            self.tetrimino = self.tetrimino.move_down(1);
        }
    }

    pub fn rotate(&mut self) {
        if self.is_end {
            return;
        }

        let new_tetrimino = self.tetrimino.rotate(1);
        let result = near_points()
            .iter()
            .map(|p| new_tetrimino.move_right(p.0).move_down(p.1))
            .find(|t| !judge::overlapping(&self.field, t));
        if let Some(t) = result {
            self.tetrimino = t;
        }
    }

    pub fn hard_drop(&mut self) {
        if self.is_end {
            return;
        }

        self.tetrimino = self.ghost();
    }

    pub fn save(&mut self) {
        if self.is_end {
            return;
        }

        for pos in self.tetrimino.blocks() {
            self.field.set(&pos, self.tetrimino.color());
        }
        if self.tetrimino.bottom() < 0 {
            self.is_end = true;
        }
        self.tetrimino = Tetrimino::new((self.selector)());
        self.init_pos();
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
