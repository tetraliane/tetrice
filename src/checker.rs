use std::collections::{HashSet, VecDeque};

use crate::field::Field;
use crate::tetrimino::Tetrimino;

pub struct Checker<'game>(pub &'game Field, pub &'game Tetrimino);

impl<'game> Checker<'game> {
    fn block_existence(&self, map: Box<dyn Fn(&(isize, isize)) -> (isize, isize)>) -> bool {
        self.1
            .blocks()
            .iter()
            .any(|pos| self.0.get_color(map(pos)) != Some(""))
    }

    pub fn touch_left(&self) -> bool {
        self.block_existence(Box::new(|(x, y)| (x - 1, *y)))
    }
    pub fn touch_right(&self) -> bool {
        self.block_existence(Box::new(|(x, y)| (x + 1, *y)))
    }
    pub fn touch_down(&self) -> bool {
        self.block_existence(Box::new(|(x, y)| (*x, y + 1)))
    }

    pub fn overlap(&self) -> bool {
        self.block_existence(Box::new(|p| *p))
    }

    pub fn route_from(&self, start: &Tetrimino) -> bool {
        route_exists(self.0, start, self.1)
    }
    pub fn route_to(&self, goal: &Tetrimino) -> bool {
        route_exists(self.0, self.1, goal)
    }
}

fn route_exists(field: &Field, start: &Tetrimino, goal: &Tetrimino) -> bool {
    let mut seen = HashSet::from([start.clone()]);
    let mut queue = VecDeque::from([start.clone()]);

    let moves: Vec<fn(&Tetrimino) -> Tetrimino> = vec![
        |t| t.move_left(1),
        |t| t.move_right(1),
        |t| t.move_down(1),
        |t| t.rotate(1),
        |t| t.rotate(2),
        |t| t.rotate(3),
    ];

    while let Some(elem) = queue.pop_front() {
        if elem == *goal {
            return true;
        }

        for f in &moves {
            let t = f(&elem);
            if !seen.contains(&t) && !Checker(field, &t).overlap() {
                queue.push_back(t.clone());
                seen.insert(t.clone());
            }
        }
    }

    false
}
