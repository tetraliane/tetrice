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
            tetrimino: Tetrimino::new(selector(), (0, 0)),
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
        self.tetrimino = self.tetrimino.move_left();
    }
    pub fn move_right(&mut self) {
        self.tetrimino = self.tetrimino.move_right();
    }
}

#[cfg(test)]
mod tests {
    use crate::{Game, Shape};

    #[test]
    fn create_10x20_field() {
        let selector = Box::new(|| Shape::T);
        let game = Game::new(10, 20, selector);
        let field = game.field();
        assert_eq!(field.width(), 10);
        assert_eq!(field.height(), 20);
    }

    #[test]
    fn set_none_to_every_cells() {
        let selector = Box::new(|| Shape::T);
        let game = Game::new(10, 20, selector);
        let field = game.field();
        assert_eq!(field.get_color(1, 2), None);
    }

    #[test]
    fn create_a_tetrimino() {
        let selector = Box::new(|| Shape::T);
        let game = Game::new(10, 20, selector);
        let tetrimino = game.tetrimino();
        assert_eq!(
            game.tetrimino().blocks(),
            [(4, -2), (3, -1), (4, -1), (5, -1)]
        );
        assert_eq!(tetrimino.color(), "purple");
    }

    #[test]
    fn move_tetrimino() {
        let selector = Box::new(|| Shape::T);
        let mut game = Game::new(10, 20, selector);
        game.move_left();
        assert_eq!(
            game.tetrimino().blocks(),
            [(3, -2), (2, -1), (3, -1), (4, -1)]
        )
    }
}
