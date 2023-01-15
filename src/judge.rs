use crate::{Field, Tetrimino};

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
