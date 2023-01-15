#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tetrimino {
    shape: Shape,
    rot: usize,
    pos: (isize, isize),
}

impl Tetrimino {
    pub(crate) fn new(shape: Shape) -> Self {
        Self {
            shape,
            rot: 0,
            pos: (0, 0),
        }
    }

    pub fn blocks(&self) -> [(isize, isize); 4] {
        self.shape
            .blocks(self.rot)
            .map(|(x, y)| (x as isize + self.pos.0, y as isize + self.pos.1))
    }

    pub fn color(&self) -> &'static str {
        self.shape.color()
    }

    pub(crate) fn width(&self) -> usize {
        let blocks = self.shape.blocks(self.rot).map(|(x, _)| x);
        (blocks.iter().max().unwrap() - blocks.iter().min().unwrap() + 1) as usize
    }
    pub(crate) fn height(&self) -> usize {
        let blocks = self.shape.blocks(self.rot).map(|(_, y)| y);
        (blocks.iter().max().unwrap() - blocks.iter().min().unwrap() + 1) as usize
    }

    fn _move(&self, pos: (isize, isize)) -> Self {
        Self {
            shape: self.shape,
            rot: self.rot,
            pos,
        }
    }

    pub(crate) fn move_to(&self, (left, top): (isize, isize)) -> Self {
        let blocks = self.blocks();
        let current_left = blocks.iter().map(|(x, _)| x).min().unwrap();
        let current_top = blocks.iter().map(|(_, y)| y).min().unwrap();
        let diff = (left - current_left, top - current_top);
        self._move((self.pos.0 + diff.0, self.pos.1 + diff.1))
    }

    pub(crate) fn move_left(&self, dist: isize) -> Self {
        self._move((self.pos.0 - dist, self.pos.1))
    }
    pub(crate) fn move_right(&self, dist: isize) -> Self {
        self._move((self.pos.0 + dist, self.pos.1))
    }
    pub(crate) fn move_down(&self, dist: isize) -> Self {
        self._move((self.pos.0, self.pos.1 + dist))
    }

    pub(crate) fn rotate(&self, times: usize) -> Self {
        Self {
            shape: self.shape,
            rot: (self.rot + times) % self.shape.num_rot(),
            pos: self.pos,
        }
    }
}

const SHAPES: [(&[[(usize, usize); 4]], &str); 7] = [
    (&[[(0, 0), (1, 0), (0, 1), (1, 1)]], "yellow"),
    (
        &[
            [(0, 1), (1, 1), (2, 1), (3, 1)],
            [(1, 0), (1, 1), (1, 2), (1, 3)],
        ],
        "lightblue",
    ),
    (
        &[
            [(0, 0), (1, 0), (1, 1), (2, 1)],
            [(2, 0), (2, 1), (1, 1), (1, 2)],
            [(2, 2), (1, 2), (1, 1), (0, 1)],
            [(0, 2), (0, 1), (1, 1), (1, 0)],
        ],
        "red",
    ),
    (
        &[
            [(1, 0), (2, 0), (0, 1), (1, 1)],
            [(2, 1), (2, 2), (1, 0), (1, 1)],
            [(1, 2), (0, 2), (2, 1), (1, 1)],
            [(0, 1), (0, 0), (1, 2), (1, 1)],
        ],
        "green",
    ),
    (
        &[
            [(0, 0), (0, 1), (1, 1), (2, 1)],
            [(2, 0), (1, 0), (1, 1), (1, 2)],
            [(2, 2), (2, 1), (1, 1), (0, 1)],
            [(0, 2), (1, 2), (1, 1), (1, 0)],
        ],
        "blue",
    ),
    (
        &[
            [(1, 0), (0, 1), (1, 1), (2, 1)],
            [(2, 1), (1, 0), (1, 1), (1, 2)],
            [(1, 2), (2, 1), (1, 1), (0, 1)],
            [(0, 1), (1, 2), (1, 1), (1, 0)],
        ],
        "purple",
    ),
    (
        &[
            [(2, 0), (0, 1), (1, 1), (2, 1)],
            [(2, 2), (1, 0), (1, 1), (1, 2)],
            [(0, 2), (2, 1), (1, 1), (0, 1)],
            [(0, 0), (1, 2), (1, 1), (1, 0)],
        ],
        "orange",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    fn data(&self) -> (&[[(usize, usize); 4]], &'static str) {
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

    fn num_rot(&self) -> usize {
        self.data().0.len()
    }

    fn blocks(&self, rot: usize) -> [(usize, usize); 4] {
        self.data().0[rot]
    }

    fn color(&self) -> &'static str {
        self.data().1
    }
}
