/// A tetrimino consisting of four dropping blocks.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tetrimino {
    kind: BlockKind,
    rot: usize,
    pos: (isize, isize),
}

impl Tetrimino {
    pub(crate) fn new(kind: BlockKind) -> Self {
        Self {
            kind,
            rot: 0,
            pos: (0, 0),
        }
    }

    /// Get the block kind.
    pub fn kind(&self) -> BlockKind {
        self.kind
    }

    /// Get a list of positions of the all blocks consisting this tetrimino.
    pub fn blocks(&self) -> [(isize, isize); 4] {
        self.kind
            .blocks(self.rot)
            .map(|(x, y)| (x as isize + self.pos.0, y as isize + self.pos.1))
    }

    /// Get the width.
    pub fn width(&self) -> usize {
        let blocks = self.kind.blocks(self.rot).map(|(x, _)| x);
        (blocks.iter().max().unwrap() - blocks.iter().min().unwrap() + 1) as usize
    }

    /// Get the height.
    pub fn height(&self) -> usize {
        let blocks = self.kind.blocks(self.rot).map(|(_, y)| y);
        (blocks.iter().max().unwrap() - blocks.iter().min().unwrap() + 1) as usize
    }

    pub(crate) fn bottom(&self) -> isize {
        self.blocks().iter().map(|(_, y)| *y).min().unwrap()
    }

    fn _move(&self, pos: (isize, isize)) -> Self {
        Self {
            kind: self.kind,
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
    pub(crate) fn move_up(&self, dist: isize) -> Self {
        self._move((self.pos.0, self.pos.1 - dist))
    }
    pub(crate) fn move_down(&self, dist: isize) -> Self {
        self._move((self.pos.0, self.pos.1 + dist))
    }

    pub(crate) fn rotate(&self, times: usize) -> Self {
        Self {
            kind: self.kind,
            rot: (self.rot + times) % self.kind.num_rot(),
            pos: self.pos,
        }
    }
}

const SHAPES: [&[[(usize, usize); 4]]; 7] = [
    &[[(0, 0), (1, 0), (0, 1), (1, 1)]],
    &[
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(1, 0), (1, 1), (1, 2), (1, 3)],
    ],
    &[
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(2, 0), (2, 1), (1, 1), (1, 2)],
        [(2, 2), (1, 2), (1, 1), (0, 1)],
        [(0, 2), (0, 1), (1, 1), (1, 0)],
    ],
    &[
        [(1, 0), (2, 0), (0, 1), (1, 1)],
        [(2, 1), (2, 2), (1, 0), (1, 1)],
        [(1, 2), (0, 2), (2, 1), (1, 1)],
        [(0, 1), (0, 0), (1, 2), (1, 1)],
    ],
    &[
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 0), (1, 0), (1, 1), (1, 2)],
        [(2, 2), (2, 1), (1, 1), (0, 1)],
        [(0, 2), (1, 2), (1, 1), (1, 0)],
    ],
    &[
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 1), (1, 0), (1, 1), (1, 2)],
        [(1, 2), (2, 1), (1, 1), (0, 1)],
        [(0, 1), (1, 2), (1, 1), (1, 0)],
    ],
    &[
        [(2, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 2), (1, 0), (1, 1), (1, 2)],
        [(0, 2), (2, 1), (1, 1), (0, 1)],
        [(0, 0), (1, 2), (1, 1), (1, 0)],
    ],
];

/// The block kind of a tetrimino.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockKind {
    O,
    I,
    Z,
    S,
    L,
    T,
    J,
}

impl BlockKind {
    /// Returns the all items as an array.
    pub fn all_as_array() -> [Self; 7] {
        [
            BlockKind::O,
            BlockKind::I,
            BlockKind::Z,
            BlockKind::S,
            BlockKind::L,
            BlockKind::T,
            BlockKind::J,
        ]
    }

    fn data(&self) -> &[[(usize, usize); 4]] {
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
        self.data().len()
    }

    fn blocks(&self, rot: usize) -> [(usize, usize); 4] {
        self.data()[rot]
    }
}
