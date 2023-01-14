pub struct Game {
    field: Field,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Game {
            field: Field { width, height },
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
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
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn create_10x20_field() {
        let game = Game::new(10, 20);
        let field = game.field();
        assert_eq!(field.width(), 10);
        assert_eq!(field.height(), 20);
    }
}
