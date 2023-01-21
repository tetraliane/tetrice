use crate::BlockKind;

const HEIGHT_NEG: usize = 7;

/// A game field.
///
/// This consists of the visible (y>0) and non-visible (y<0) areas.
pub struct Field {
    state: Vec<Vec<Cell>>,
}

impl Field {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self::from_vec(vec![vec![Cell::Empty; width]; height + HEIGHT_NEG])
    }

    pub(crate) fn from_vec(state: Vec<Vec<Cell>>) -> Self {
        Self { state }
    }

    /// Get the visible area as an 2D-Vec.
    ///
    /// The value at every position is one of the tetrimino colors if a block
    /// exists, and an empty string otherwise.
    pub fn as_vec(&self) -> &[Vec<Cell>] {
        &self.state[HEIGHT_NEG..]
    }

    /// Get the width.
    pub fn width(&self) -> usize {
        self.state[0].len()
    }

    /// Get the height of the visible area.
    pub fn height(&self) -> usize {
        self.state.len() - HEIGHT_NEG
    }

    /// Get the color at the given position. If there are no blocks, returns
    /// `Cell::None`. If the position is out of this field, returns `Cell::Outside`.
    pub fn get_cell(&self, (x, y): (isize, isize)) -> Cell {
        let width = self.width() as isize;
        let height_min = -(HEIGHT_NEG as isize);
        let height_max = self.height() as isize;
        if (0..width).contains(&x) && (height_min..height_max).contains(&y) {
            let x = x as usize;
            let y = (y + HEIGHT_NEG as isize) as usize;
            self.state[y][x]
        } else {
            Cell::Outside
        }
    }

    pub(crate) fn set(&mut self, (x, y): (isize, isize), kind: BlockKind) {
        let x = x as usize;
        let y = (y + HEIGHT_NEG as isize) as usize;
        self.state[y][x] = Cell::Block(kind);
    }

    pub(crate) fn remove_filled_lines(&mut self) -> usize {
        let lines_not_filled: Vec<_> = self
            .state
            .iter()
            .filter(|line| !line.iter().all(|cell| *cell != Cell::Empty))
            .cloned()
            .collect();
        let count = self.state.len() - lines_not_filled.len();

        self.state = [vec![vec![Cell::Empty; 10]; count], lines_not_filled].concat();

        count
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_vec().iter().try_for_each(|row| {
            write!(f, "|").and_then(|()| {
                row.iter()
                    .try_for_each(|cell| match cell {
                        Cell::Block(kind) => kind.fmt(f),
                        Cell::Empty => write!(f, "_"),
                        Cell::Outside => write!(f, " "),
                    })
                    .and_then(|()| writeln!(f, "|"))
            })
        })
    }
}

/// A state of cells in the field.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    /// Indicates there is a block in the cell. The kind is included as the value.
    Block(BlockKind),
    /// Indicates there is no block in the cell.
    Empty,
    /// Indicates the specified cell is out of the field.
    Outside,
}
