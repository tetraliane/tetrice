pub struct Field {
    width: usize,
    height: usize,
}

impl Field {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

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
