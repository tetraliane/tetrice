use std::collections::{HashSet, VecDeque};

use crate::field::Field;
use crate::tetrimino::Tetrimino;

fn check_block_existence(
    field: &Field,
    tetrimino: &Tetrimino,
    map: Box<dyn Fn(&(isize, isize)) -> (isize, isize)>,
) -> bool {
    tetrimino
        .blocks()
        .iter()
        .any(|pos| field.get_color(map(pos)) != Some(""))
}

pub(crate) fn touching_left(field: &Field, tetrimino: &Tetrimino) -> bool {
    check_block_existence(field, tetrimino, Box::new(|(x, y)| (x - 1, *y)))
}
pub(crate) fn touching_right(field: &Field, tetrimino: &Tetrimino) -> bool {
    check_block_existence(field, tetrimino, Box::new(|(x, y)| (x + 1, *y)))
}
pub(crate) fn touching_down(field: &Field, tetrimino: &Tetrimino) -> bool {
    check_block_existence(field, tetrimino, Box::new(|(x, y)| (*x, y + 1)))
}

pub(crate) fn overlapping(field: &Field, tetrimino: &Tetrimino) -> bool {
    check_block_existence(field, tetrimino, Box::new(|p| *p))
}

pub(crate) fn route_exists(field: &Field, start: &Tetrimino, goal: &Tetrimino) -> bool {
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
            if !seen.contains(&t) && !overlapping(field, &t) {
                queue.push_back(t.clone());
                seen.insert(t.clone());
            }
        }
    }

    false
}
