const HEIGHT_NEG: usize = 7;

/// A game field.
///
/// This consists of the visible (y>0) and non-visible (y<0) areas.
#[derive(Debug)]
pub struct Field {
    state: Vec<Vec<&'static str>>,
}

impl Field {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self::from_vec(vec![vec![""; width]; height + HEIGHT_NEG])
    }

    pub(crate) fn from_vec(state: Vec<Vec<&'static str>>) -> Self {
        Self { state }
    }

    /// Get the visible area as an 2D-Vec.
    ///
    /// The value at every position is one of the tetrimino colors if a block
    /// exists, and an empty string otherwise.
    pub fn as_vec(&self) -> &[Vec<&str>] {
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

    /// Get the color at the given position. If there are no blocks, returns an
    /// empty string. If the position is out of this field, returns `None`.
    pub fn get_color(&self, (x, y): (isize, isize)) -> Cell {
        let width = self.width() as isize;
        let height_min = -(HEIGHT_NEG as isize);
        let height_max = self.height() as isize;
        if (0..width).contains(&x) && (height_min..height_max).contains(&y) {
            let x = x as usize;
            let y = (y + HEIGHT_NEG as isize) as usize;
            if self.state[y][x] == "" {
                Cell::Empty
            } else {
                Cell::Block(self.state[y][x])
            }
        } else {
            Cell::Outside
        }
    }

    pub(crate) fn set(&mut self, (x, y): (isize, isize), color: &'static str) {
        let x = x as usize;
        let y = (y + HEIGHT_NEG as isize) as usize;
        self.state[y][x] = color;
    }

    pub(crate) fn remove_filled_lines(&mut self) -> usize {
        let lines_not_filled: Vec<_> = self
            .state
            .iter()
            .filter(|line| !line.iter().all(|cell| *cell != ""))
            .map(|line| line.clone())
            .collect();
        let count = self.state.len() - lines_not_filled.len();

        self.state = [vec![vec![""; 10]; count], lines_not_filled].concat();

        count
    }
}

#[derive(Debug, PartialEq)]
pub enum Cell {
    Empty,
    Block(&'static str),
    Outside,
}
