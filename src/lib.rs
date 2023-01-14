pub struct Game {
    field: Field,
    tetrimino: Tetrimino,
}

impl Game {
    pub fn new(width: usize, height: usize, selector: Box<dyn Fn() -> Shape>) -> Self {
        Game {
            field: Field { width, height },
            tetrimino: Tetrimino { shape: selector() }
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn tetrimino(&self) -> &Tetrimino {
        &self.tetrimino
    }
}

pub struct Field {
    width: usize,
    height: usize,
}

impl Field {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_color(&self, _: usize, _: usize) -> Option<String> {
        None
    }
}

pub struct Tetrimino {
    shape: Shape,
}

impl Tetrimino {
    pub fn blocks(&self) -> [(usize, usize); 4] {
        self.shape.blocks()
    }

    pub fn color(&self) -> &str {
        self.shape.color()
    }
}

const SHAPES: [([(usize, usize); 4], &str); 7] = [
    ([(0, 0), (1, 0), (0, 1), (1, 1)], "yellow"),
    ([(0, 0), (1, 0), (2, 0), (3, 0)], "lightblue"),
    ([(0, 0), (1, 0), (1, 1), (2, 1)], "red"),
    ([(1, 0), (2, 0), (0, 1), (1, 1)], "green"),
    ([(1, 0), (1, 1), (1, 2), (2, 2)], "orange"),
    ([(1, 0), (0, 1), (1, 1), (2, 1)], "purple"),
    ([(1, 0), (1, 1), (0, 2), (1, 2)], "blue"),
];

pub enum Shape {
    O,
    I,
    Z,
    S,
    L,
    T,
    J,
}

impl Shape {
    fn data(&self) -> ([(usize, usize); 4], &str) {
        match &self {
            Self::O => SHAPES[0],
            Self::I => SHAPES[1],
            Self::S => SHAPES[2],
            Self::Z => SHAPES[3],
            Self::L => SHAPES[4],
            Self::T => SHAPES[5],
            Self::J => SHAPES[6],
        }
    }

    fn blocks(&self) -> [(usize, usize); 4] {
        self.data().0
    }

    fn color(&self) -> &str {
        self.data().1
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
