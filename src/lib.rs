//! This crate provides core functions of Tetris.
//!
//! # How to use
//!
//! 1. Import `Game` and `Shape`.
//! 2. Make a shape selector.
//!    ```ignore
//!    fn selector() -> Shape {
//!        // Return one of the shapes (probably you want to select randomly)
//!    }
//!    ```
//! 3. Create a game.
//!    ```ignore
//!    fn main() {
//!        // Create a game which has a 10x20 field and provides 3 next tetriminos
//!        let mut game = Game::new(10, 20, 3, selector);
//!
//!        // Now you can move, rotate, etc. using `game`!
//!    }
//!    ```

mod checker;
mod field;
mod tetrimino;

#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::VecDeque;

pub use checker::Checker;
pub use field::{Cell, Field};
pub use tetrimino::{Shape, Tetrimino};

/// A game manager.
///
/// Move or rotate the tetrimino so that it is always inside the field and it
/// doesn't overlap to other blocks. Also have "next tetrimino" and "hold" system.
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
    /// Create a new game.
    ///
    /// `selector` is a function called when creating a new tetrimino.
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

    /// Get the field.
    pub fn field(&self) -> &Field {
        &self.field
    }

    /// Get the current tetrimino.
    pub fn tetrimino(&self) -> &Tetrimino {
        &self.tetrimino
    }

    /// Get the queue of next tetriminos.
    pub fn queue(&self) -> &VecDeque<Tetrimino> {
        &self.queue
    }

    /// Get the held tetrimino. If no tetrimino is held, returns `None`.
    pub fn held(&self) -> Option<Tetrimino> {
        self.held.clone()
    }

    /// Get the ghost, which shows a tetrimino after the current tetrimino is
    /// hard-dropped.
    ///
    /// The ghost is located the deepest place the current tetrimino can reach
    /// by moving left, right or down and rotating. For example, consider
    /// the field like this (`x` is the blocks in the field and `o` is the
    /// blocks of the tetrimino):
    ///
    /// ```txt
    /// |    o   |
    /// |   ooo  |
    /// |        |
    /// |   xxxxx|
    /// |       x|
    /// |       x|
    /// ```
    ///
    /// Here the ghost is located like the following because the tetrimino can
    /// reach there by going left, then down, and then right.
    ///
    /// ```txt
    /// |        |
    /// |        |
    /// |        |
    /// |   xxxxx|
    /// |    o  x|
    /// |   ooo x|
    /// ```
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

    /// Create a `Checker` from the field and the current tetrimino.
    pub fn check(&self) -> Checker {
        Checker(&self.field, &self.tetrimino)
    }

    /// Returns true if this game has ended.
    ///
    /// The game ends when a tetrimino is saved completely in the non-visible
    /// area. After the end, this game manager doesn't change the tetrimino
    /// and the field anymore. If even one of the four blocks is saved in the
    /// visible area, the game continues.
    pub fn is_end(&self) -> bool {
        return self.is_end;
    }

    /// Get the number of lines removed in this game.
    pub fn removed_lines(&self) -> usize {
        self.removed_lines
    }

    /// Move the current tetrimino to the left. However, when it touches the
    /// left border or other blocks, or after the game has end, do nothing.
    ///
    /// Returns true when actually moved the tetrimino.
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

    /// Same as `move_left`, but move the tetrimino to the right.
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

    /// Same as `move_left`, but move down the tetrimino.
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

    /// Rotate the tetrimino clockwise, and move it to where it doesn't
    /// overlap. However do nothing when such a place doesn't exist nearby or
    /// after the game has end.
    ///
    /// Returns true if actually rotated the tetrimino.
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

    /// Drop the tetrimino to the position of the ghost. Doesn't work after end.
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

    /// Save the current tetrimino to the field and remove the filled lines.
    /// Returns the number of removed lines.
    ///
    /// Doesn't work after end.
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

    /// Hold the current tetrimino. Doesn't work just after another holding or
    /// after the game has ended. Returns true when holding has been executed.
    ///
    /// Note: You can't hold tetriminos twice without saving.
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
