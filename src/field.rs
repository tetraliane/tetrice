const HEIGHT_NEG: usize = 7;

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

    pub fn width(&self) -> usize {
        self.state[0].len()
    }

    pub fn height(&self) -> usize {
        self.state.len() - HEIGHT_NEG
    }

    pub fn get_color(&self, (x, y): (isize, isize)) -> Option<&str> {
        if x < 0 || y < -(HEIGHT_NEG as isize) {
            None
        } else {
            let x = x as usize;
            let y = (y + HEIGHT_NEG as isize) as usize;
            self.state.get(y).and_then(|row| row.get(x).map(|c| *c))
        }
    }
}
