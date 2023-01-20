use std::collections::HashSet;

use crate::{
    field::{Cell, Field},
    tetrimino::{BlockKind, Tetrimino},
    Game,
};

fn make_selector() -> Box<dyn FnMut() -> BlockKind> {
    let mut count = 0;
    Box::new(move || {
        count += 1;
        match count {
            1 => BlockKind::T,
            2 => BlockKind::L,
            3 => BlockKind::I,
            _ => BlockKind::J,
        }
    })
}

fn make_game() -> Game {
    Game::new(10, 20, 3, make_selector())
}

#[test]
fn create_10x20_field() {
    let game = make_game();
    let field = game.field();
    assert_eq!(field.width(), 10);
    assert_eq!(field.height(), 20);
}

#[test]
#[should_panic]
fn width_must_be_4_or_more() {
    Game::new(3, 20, 3, make_selector());
}

#[test]
#[should_panic]
fn height_must_be_1_or_more() {
    Game::new(10, 0, 3, make_selector());
}

#[test]
fn set_empty_string_to_every_cells() {
    let game = make_game();
    let field = game.field();
    assert_eq!(field.get_cell((1, 2)), Cell::Empty);
}

#[test]
fn return_color_for_saved_blocks() {
    let mut game = make_game();
    game.hard_drop();
    game.save();
    assert_eq!(game.field().get_cell((4, 19)), Cell::Block(BlockKind::T))
}

#[test]
fn do_not_panic_for_outside_points() {
    let game = make_game();
    let field = game.field();
    assert_eq!(field.get_cell((-1, 2)), Cell::Outside);
}

#[test]
fn field_includes_points_above_the_visible_area() {
    let game = make_game();
    let field = game.field();
    assert_eq!(field.get_cell((1, -2)), Cell::Empty);
}

#[test]
fn create_a_tetrimino() {
    let game = make_game();
    let tetrimino = game.tetrimino();
    // Locate the tetrimino at the center above the field
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::T).move_to((3, -2))
    );
    assert_eq!(tetrimino.kind(), BlockKind::T);
}

#[test]
fn locate_the_tetrimino_higher_when_it_overlaps() {
    let mut game = make_game();
    game.field = Field::from_vec(
        [
            vec![vec![Cell::Empty; 10]; 6],
            vec![vec![Cell::Block(BlockKind::O); 10]],
            vec![vec![Cell::Empty; 10]; 20],
        ]
        .concat(),
    );
    game.tetrimino = Tetrimino::new(BlockKind::T).move_to((3, 18));

    game.save();
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::L).move_to((3, -3))
    );
}

#[test]
fn create_queue_of_three_tetriminos() {
    let game = make_game();
    assert_eq!(
        game.queue(),
        &[
            Tetrimino::new(BlockKind::L),
            Tetrimino::new(BlockKind::I),
            Tetrimino::new(BlockKind::J),
        ]
    )
}

#[test]
fn do_not_hold_any_tetrimino_at_first() {
    let game = make_game();
    let held = game.held();
    assert!(held.is_none());
}

#[test]
fn move_tetrimino_to_left() {
    let mut game = make_game();
    let result = game.move_left();
    // The tetrimino moves to left by one step from (3, -2)
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::T).move_to((3 - 1, -2)),
    );
    assert_eq!(result, true);
}

#[test]
fn move_tetrimino_to_right() {
    let mut game = make_game();
    let result = game.move_right();
    // The tetrimino moves to right by one step from (3, -2)
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::T).move_to((3 + 1, -2))
    );
    assert_eq!(result, true);
}

#[test]
fn soft_drop() {
    let mut game = make_game();
    let result = game.soft_drop();
    // The tetrimino moves down by one step from (3, -2)
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::T).move_to((3, -2 + 1))
    );
    assert_eq!(result, true);
}

#[test]
fn do_not_go_through_border() {
    let original = Tetrimino::new(BlockKind::T).move_to((0, 0));

    let mut game = make_game();
    game.tetrimino = original.clone();

    let result = game.move_left();
    assert_eq!(game.tetrimino(), &original);
    assert_eq!(result, false);
}

#[test]
fn do_not_go_through_other_blocks() {
    let original = Tetrimino::new(BlockKind::T).move_to((1, 0));

    // 7 is the height of the negative area
    let mut field = vec![vec![Cell::Empty; 4]; 7 + 1];
    field.push(vec![
        Cell::Block(BlockKind::O),
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ]);
    let mut game = make_game();
    game.field = Field::from_vec(field);
    game.tetrimino = original.clone();

    let result = game.move_left();
    assert_eq!(game.tetrimino(), &original);
    assert_eq!(result, false);
}

#[test]
fn rotate_tetrimino() {
    let mut game = make_game();
    let result = game.rotate();
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::T).move_to((3, -2)).rotate(1)
    );
    assert_eq!(result, true);
}

#[test]
fn move_tetrimino_not_to_overlap_after_rotation() {
    // 7 is the height of the negative area
    let mut game = make_game();
    game.field = Field::from_vec(vec![vec![Cell::Empty; 3]; 7 + 2]);
    game.tetrimino = Tetrimino::new(BlockKind::T);

    game.rotate();
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::T).rotate(1).move_up(1)
    );
}

#[test]
fn create_ghost() {
    let game = make_game();
    assert_eq!(game.ghost(), Tetrimino::new(BlockKind::T).move_to((3, 18)));
}

#[test]
fn ghost_may_jump_over_blocks() {
    // 7 is the height of the negative area
    let field_state = [
        vec![vec![Cell::Empty; 10]; 7],
        vec![vec![
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Block(BlockKind::O),
            Cell::Block(BlockKind::O),
            Cell::Block(BlockKind::O),
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ]],
        vec![vec![Cell::Empty; 10]; 3],
    ]
    .concat();
    let mut game = make_game();
    game.field = Field::from_vec(field_state);

    assert_eq!(game.ghost(), Tetrimino::new(BlockKind::T).move_to((3, 2)));
}

#[test]
fn hard_drop_tetrimino() {
    let mut game = make_game();
    game.hard_drop();
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::T).move_to((3, 18)),
    )
}

#[test]
fn save_tetrimino() {
    let mut game = make_game();
    game.hard_drop();
    game.save();
    assert_eq!(
        game.field().as_vec()[17..],
        [
            [Cell::Empty; 10],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Block(BlockKind::T),
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Block(BlockKind::T),
                Cell::Block(BlockKind::T),
                Cell::Block(BlockKind::T),
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty
            ],
        ]
    );
    // L-tetrimino is generated
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::L).move_to((3, -2))
    );
}

#[test]
fn remove_filled_lines_when_saving() {
    let mut game = make_game();
    game.field = Field::from_vec(
        [
            vec![vec![Cell::Empty; 10]; 26],
            vec![vec![
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
            ]],
        ]
        .concat(),
    );

    game.hard_drop();
    let result = game.save();
    assert_eq!(
        game.field().as_vec()[18..],
        [
            [Cell::Empty; 10],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Block(BlockKind::T),
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty
            ],
        ]
    );
    // return how many lines are removed
    assert_eq!(result, 1);
}

#[test]
fn end_when_saved_tetrimino_is_out_of_visible_area() {
    let mut game = make_game();
    game.field = Field::from_vec(vec![vec![Cell::Empty; 10]; 7]);

    game.save();
    assert!(game.is_end());
}

#[test]
fn stop_updating_after_end() {
    let mut game = make_game();
    game.is_end = true;

    let expected = Tetrimino::new(BlockKind::T).move_to((3, -2));
    game.move_left();
    assert_eq!(game.tetrimino(), &expected);
    game.move_right();
    assert_eq!(game.tetrimino(), &expected);
    game.soft_drop();
    assert_eq!(game.tetrimino(), &expected);
    game.rotate();
    assert_eq!(game.tetrimino(), &expected);
    game.hard_drop();
    assert_eq!(game.tetrimino(), &expected);
    game.save();
    assert_eq!(game.field().as_vec(), vec![vec![Cell::Empty; 10]; 20]);
    game.hold();
    assert_eq!(game.tetrimino(), &expected);
    assert!(game.held().is_none());
}

#[test]
fn hold_tetrimino() {
    let mut game = make_game();
    game.hold();
    assert_eq!(
        game.held().unwrap(),
        Tetrimino::new(BlockKind::T).move_to((0, 0))
    );
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::L).move_to((3, -2))
    );
}

#[test]
fn reset_rotation_when_holding() {
    let mut game = make_game();
    game.rotate();
    game.hold();
    assert_eq!(
        game.held().unwrap(),
        Tetrimino::new(BlockKind::T).move_to((0, 0))
    );
}

#[test]
fn do_not_hold_twice_without_saving() {
    let mut game = make_game();
    game.hold();
    game.hold(); // actually doesn't hold
    assert_eq!(
        game.held().unwrap(),
        Tetrimino::new(BlockKind::T).move_to((0, 0))
    );
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::L).move_to((3, -2))
    );
}

#[test]
fn can_hold_again_after_save() {
    let mut game = make_game();
    game.hold(); // held: T, current: L
    game.hard_drop();
    game.save(); // held: T, current: I
    game.hold(); // held: I, current: T
    assert_eq!(
        game.held().unwrap(),
        Tetrimino::new(BlockKind::I).move_to((0, 0))
    );
    assert_eq!(
        game.tetrimino(),
        &Tetrimino::new(BlockKind::T).move_to((3, -2))
    );
}

#[test]
fn have_sum_of_removed_lines() {
    let mut game = make_game();
    game.field = Field::from_vec(
        [
            vec![vec![Cell::Empty; 10]; 26],
            vec![vec![
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
                Cell::Block(BlockKind::O),
            ]],
        ]
        .concat(),
    );

    assert_eq!(game.removed_lines(), 0);
    game.hard_drop();
    game.save();
    assert_eq!(game.removed_lines(), 1);
}

#[test]
fn make_list_of_all_kinds() {
    let result = BlockKind::all_as_array();
    assert_eq!(
        HashSet::from(result),
        HashSet::from([
            BlockKind::I,
            BlockKind::J,
            BlockKind::L,
            BlockKind::O,
            BlockKind::S,
            BlockKind::T,
            BlockKind::Z,
        ])
    )
}

#[test]
fn implement_debug() {
    format!("{:?}", BlockKind::T);
    format!("{:?}", Tetrimino::new(BlockKind::T));
    format!("{:?}", Field::new(10, 20));
}
