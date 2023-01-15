pub struct Field {
    state: Vec<Vec<&'static str>>,
}

impl Field {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self::from_vec(vec![vec![""; width]; height])
    }

    pub(crate) fn from_vec(state: Vec<Vec<&'static str>>) -> Self {
        Self { state }
    }

    pub fn width(&self) -> usize {
        self.state[0].len()
    }

    pub fn height(&self) -> usize {
        self.state.len()
    }

    pub fn get_color(&self, x: usize, y: usize) -> Option<&str> {
        self.state.get(y).and_then(|row| row.get(x).map(|c| *c))
    }
}
