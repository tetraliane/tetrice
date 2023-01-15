pub struct Field {
    state: Vec<Vec<Option<&'static str>>>,
}

impl Field {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self::from_vec(vec![vec![None; width]; height])
    }

    pub(crate) fn from_vec(state: Vec<Vec<Option<&'static str>>>) -> Self {
        Self { state }
    }

    pub fn width(&self) -> usize {
        self.state[0].len()
    }

    pub fn height(&self) -> usize {
        self.state.len()
    }

    pub fn get_color(&self, _: usize, _: usize) -> Option<String> {
        None
    }
}
