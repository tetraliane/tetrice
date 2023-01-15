mod field;
mod tetrimino;

use field::Field;
use tetrimino::{Shape, Tetrimino};

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
            self.tetrimino = self.tetrimino.move_left();
        }
    }
    pub fn move_right(&mut self) {
        if !self.touching_right() {
            self.tetrimino = self.tetrimino.move_right();
        }
    }
    pub fn soft_drop(&mut self) {
        if !self.touching_down() {
            self.tetrimino = self.tetrimino.move_down();
        }
    }

    pub fn rotate(&mut self) {
        self.tetrimino = self.tetrimino.rotate();
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
}
