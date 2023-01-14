mod field;
mod tetrimino;

use field::Field;
use tetrimino::{Shape, Tetrimino};

pub struct Game {
    field: Field,
    tetrimino: Tetrimino,
}

impl Game {
    pub fn new(width: usize, height: usize, selector: Box<dyn Fn() -> Shape>) -> Self {
        Game {
            field: Field::new(width, height),
            tetrimino: Tetrimino::new(selector())
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn tetrimino(&self) -> &Tetrimino {
        &self.tetrimino
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
        assert_eq!(tetrimino.blocks(), [(1, 0), (0, 1), (1, 1), (2, 1)]);
        assert_eq!(tetrimino.color(), "purple");
    }
}
