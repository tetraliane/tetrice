mod field;
mod tetrimino;

use std::cmp::Ordering;

use field::Field;
use tetrimino::{Shape, Tetrimino};

fn overlapping(field: &Field, tetrimino: &Tetrimino) -> bool {
    tetrimino
        .blocks()
        .iter()
        .any(|pos| field.get_color(*pos) != Some(""))
}

pub struct Game {
    field: Field,
    tetrimino: Tetrimino,
    selector: Box<dyn Fn() -> Shape>,
}

impl Game {
    pub fn new(width: usize, height: usize, selector: Box<dyn Fn() -> Shape>) -> Self {
        let mut game = Game {
            field: Field::new(width, height),
            tetrimino: Tetrimino::new(selector()),
            selector,
        };
        game.init_pos();
        game
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn tetrimino(&self) -> &Tetrimino {
        &self.tetrimino
    }

    fn init_pos(&mut self) {
        self.tetrimino = self.tetrimino.move_to((
            (self.field.width() - self.tetrimino.width()) as isize / 2,
            -1 * self.tetrimino.height() as isize,
        ))
    }

    fn _check_block_existence(&self, map: Box<dyn Fn(&(isize, isize)) -> (isize, isize)>) -> bool {
        self.tetrimino
            .blocks()
            .iter()
            .any(|pos| self.field.get_color(map(pos)) != Some(""))
    }
    fn touching_left(&self) -> bool {
        self._check_block_existence(Box::new(|(x, y)| (x - 1, *y)))
    }
    fn touching_right(&self) -> bool {
        self._check_block_existence(Box::new(|(x, y)| (x + 1, *y)))
    }
    fn touching_down(&self) -> bool {
        self._check_block_existence(Box::new(|(x, y)| (*x, y + 1)))
    }

    pub fn move_left(&mut self) {
        if !self.touching_left() {
            self.tetrimino = self.tetrimino.move_left(1);
        }
    }
    pub fn move_right(&mut self) {
        if !self.touching_right() {
            self.tetrimino = self.tetrimino.move_right(1);
        }
    }
    pub fn soft_drop(&mut self) {
        if !self.touching_down() {
            self.tetrimino = self.tetrimino.move_down(1);
        }
    }

    pub fn rotate(&mut self) {
        let new_tetrimino = self.tetrimino.rotate();
        let result = near_points()
            .iter()
            .map(|p| new_tetrimino.move_right(p.0).move_down(p.1))
            .find(|t| !overlapping(&self.field, t));
        if let Some(t) = result {
            self.tetrimino = t;
        }
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

#[cfg(test)]
mod tests {
    use crate::{Game, Shape};

    fn make_game() -> Game {
        let selector = Box::new(|| Shape::T);
        Game::new(10, 20, selector)
    }

    #[test]
    fn create_10x20_field() {
        let game = make_game();
        let field = game.field();
        assert_eq!(field.width(), 10);
        assert_eq!(field.height(), 20);
    }

    #[test]
    fn set_empty_string_to_every_cells() {
        let game = make_game();
        let field = game.field();
        assert_eq!(field.get_color((1, 2)), Some(""));
    }

    #[test]
    fn return_none_for_outside_points() {
        let game = make_game();
        let field = game.field();
        assert_eq!(field.get_color((-1, 2)), None);
    }

    #[test]
    fn return_some_for_points_above_the_field() {
        let game = make_game();
        let field = game.field();
        assert_eq!(field.get_color((1, -2)), Some(""));
    }

    #[test]
    fn create_a_tetrimino() {
        let game = make_game();
        let tetrimino = game.tetrimino();
        assert_eq!(
            game.tetrimino().blocks(),
            [(4, -2), (3, -1), (4, -1), (5, -1)]
        );
        assert_eq!(tetrimino.color(), "purple");
    }

    #[test]
    fn move_tetrimino() {
        let mut game = make_game();
        game.move_left();
        assert_eq!(
            game.tetrimino().blocks(),
            [(3, -2), (2, -1), (3, -1), (4, -1)]
        )
    }

    #[test]
    fn soft_drop() {
        let mut game = make_game();
        game.soft_drop();
        assert_eq!(game.tetrimino().blocks(), [(4, -1), (3, 0), (4, 0), (5, 0)])
    }

    #[test]
    fn do_not_go_through_border() {
        let mut game = Game {
            field: crate::Field::new(10, 20),
            tetrimino: crate::Tetrimino::new(Shape::T).move_to((0, 0)),
            selector: Box::new(|| Shape::T),
        };
        game.move_left();
        assert_eq!(game.tetrimino().blocks(), [(1, 0), (0, 1), (1, 1), (2, 1)]);
    }

    #[test]
    fn do_not_go_through_other_blocks() {
        let field = vec![vec!["", "", "", ""], vec!["red", "", "", ""]];
        let mut game = Game {
            field: crate::Field::from_vec(field),
            tetrimino: crate::Tetrimino::new(Shape::T).move_to((1, 0)),
            selector: Box::new(|| Shape::T),
        };
        game.move_left();
        assert_eq!(game.tetrimino().blocks(), [(2, 0), (1, 1), (2, 1), (3, 1)]);
    }

    #[test]
    fn rotate_tetrimino() {
        let mut game = make_game();
        game.rotate();
        assert_eq!(
            game.tetrimino().blocks(),
            [(5, -1), (4, -2), (4, -1), (4, 0)]
        )
    }

    #[test]
    fn move_tetrimino_not_to_overlap_after_rotation() {
        let field = vec![vec!["", "", ""]; 9];
        let mut game = Game {
            field: crate::Field::from_vec(field),
            tetrimino: crate::Tetrimino::new(Shape::T).move_to((0, 0)),
            selector: Box::new(|| Shape::T),
        };
        game.rotate();
        assert_eq!(game.tetrimino().blocks(), [(2, 0), (1, -1), (1, 0), (1, 1)]);
    }
}
