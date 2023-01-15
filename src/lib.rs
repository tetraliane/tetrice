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

    pub fn move_left(&mut self) {
        if self.tetrimino.left() > 0 {
            self.tetrimino = self.tetrimino.move_left();
        }
    }
    pub fn move_right(&mut self) {
        if self.tetrimino.right() < self.field.width() as isize - 1 {
            self.tetrimino = self.tetrimino.move_right();
        }
    }
    pub fn soft_drop(&mut self) {
        if self.tetrimino.bottom() < self.field.height() as isize - 1 {
            self.tetrimino = self.tetrimino.move_down();
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
    fn set_none_to_every_cells() {
        let game = make_game();
        let field = game.field();
        assert_eq!(field.get_color(1, 2), None);
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
}
