pub struct Tetrimino {
    shape: Shape,
}

impl Tetrimino {
    pub(crate) fn new(shape: Shape) -> Self {
        Self { shape }
    }

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
