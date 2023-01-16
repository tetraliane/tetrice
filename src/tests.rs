use crate::{
    field::Field,
    tetrimino::{Shape, Tetrimino},
    Game,
};

fn make_game() -> Game {
    let mut count = 0;
    let selector = Box::new(move || {
        count += 1;
        match count {
            1 => Shape::T,
            2 => Shape::L,
            3 => Shape::I,
            _ => Shape::J,
        }
    });
    Game::new(10, 20, 3, selector)
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
    let selector = Box::new(|| Shape::T);
    Game::new(3, 20, 3, selector);
}

#[test]
#[should_panic]
fn height_must_be_1_or_more() {
    let selector = Box::new(|| Shape::T);
    Game::new(10, 0, 3, selector);
}

#[test]
fn set_empty_string_to_every_cells() {
    let game = make_game();
    let field = game.field();
    assert_eq!(field.get_color((1, 2)), Some(""));
}

#[test]
fn return_none_for_outside_points() {
    let game = make_game();
    let field = game.field();
    assert_eq!(field.get_color((-1, 2)), None);
}

#[test]
fn return_some_for_points_above_the_field() {
    let game = make_game();
    let field = game.field();
    assert_eq!(field.get_color((1, -2)), Some(""));
}

#[test]
fn create_a_tetrimino() {
    let game = make_game();
    let tetrimino = game.tetrimino();
    assert_eq!(
        game.tetrimino().blocks(),
        [(4, -2), (3, -1), (4, -1), (5, -1)]
    );
    assert_eq!(tetrimino.color(), "purple");
}

#[test]
fn create_queue_of_three_tetriminos() {
    let game = make_game();
    assert_eq!(
        game.queue(),
        &[
            Tetrimino::new(Shape::L),
            Tetrimino::new(Shape::I),
            Tetrimino::new(Shape::J),
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
fn move_tetrimino() {
    let mut game = make_game();
    game.move_left();
    assert_eq!(
        game.tetrimino().blocks(),
        [(3, -2), (2, -1), (3, -1), (4, -1)]
    )
}

#[test]
fn soft_drop() {
    let mut game = make_game();
    game.soft_drop();
    assert_eq!(game.tetrimino().blocks(), [(4, -1), (3, 0), (4, 0), (5, 0)])
}

#[test]
fn do_not_go_through_border() {
    let mut game = make_game();
    game.tetrimino = Tetrimino::new(Shape::T).move_to((0, 0));

    game.move_left();
    assert_eq!(game.tetrimino().blocks(), [(1, 0), (0, 1), (1, 1), (2, 1)]);
}

#[test]
fn do_not_go_through_other_blocks() {
    // 7 is the height of the negative area
    let mut field = vec![vec![""; 4]; 7 + 1];
    field.push(vec!["red", "", "", ""]);
    let mut game = make_game();
    game.field = Field::from_vec(field);
    game.tetrimino = Tetrimino::new(Shape::T).move_to((1, 0));

    game.move_left();
    assert_eq!(game.tetrimino().blocks(), [(2, 0), (1, 1), (2, 1), (3, 1)]);
}

#[test]
fn rotate_tetrimino() {
    let mut game = make_game();
    game.rotate();
    assert_eq!(
        game.tetrimino().blocks(),
        [(5, -1), (4, -2), (4, -1), (4, 0)]
    )
}

#[test]
fn move_tetrimino_not_to_overlap_after_rotation() {
    // 7 is the height of the negative area
    let mut game = make_game();
    game.field = Field::from_vec(vec![vec![""; 3]; 7 + 2]);
    game.tetrimino = Tetrimino::new(Shape::T);

    game.rotate();
    assert_eq!(game.tetrimino().blocks(), [(2, 0), (1, -1), (1, 0), (1, 1)]);
}

#[test]
fn create_ghost() {
    let game = make_game();
    let ghost = game.ghost();
    assert_eq!(ghost.blocks(), [(4, 18), (3, 19), (4, 19), (5, 19)]);
}

#[test]
fn ghost_may_jump_over_blocks() {
    // 7 is the height of the negative area
    let field_state = [
        vec![vec![""; 10]; 7],
        vec![vec!["", "", "", "red", "red", "red", "", "", "", ""]],
        vec![vec![""; 10]; 3],
    ]
    .concat();
    let mut game = make_game();
    game.field = Field::from_vec(field_state);

    let ghost = game.ghost();
    assert_eq!(ghost.blocks(), [(4, 2), (3, 3), (4, 3), (5, 3)]);
}

#[test]
fn hard_drop_tetrimino() {
    let mut game = make_game();
    game.hard_drop();
    assert_eq!(
        game.tetrimino().blocks(),
        [(4, 18), (3, 19), (4, 19), (5, 19)]
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
            [""; 10],
            ["", "", "", "", "purple", "", "", "", "", ""],
            ["", "", "", "purple", "purple", "purple", "", "", "", ""],
        ]
    );
    // L-tetrimino is generated
    assert_eq!(
        game.tetrimino().blocks(),
        [(3, -2), (3, -1), (4, -1), (5, -1)]
    );
}

#[test]
fn remove_filled_lines_when_saving() {
    let mut game = make_game();
    game.field = Field::from_vec(
        [
            vec![vec![""; 10]; 26],
            vec![vec![
                "red", "red", "red", "", "", "", "red", "red", "red", "red",
            ]],
        ]
        .concat(),
    );

    game.hard_drop();
    let result = game.save();
    assert_eq!(
        game.field().as_vec()[18..],
        [[""; 10], ["", "", "", "", "purple", "", "", "", "", ""],]
    );
    // return how many lines are removed
    assert_eq!(result, 1);
}

#[test]
fn end_when_saved_tetrimino_is_out_of_visible_area() {
    let mut game = make_game();
    game.field = Field::from_vec(vec![vec![""; 10]; 7]);

    game.save();
    assert!(game.is_end());
}

#[test]
fn stop_updating_after_end() {
    let mut game = make_game();
    game.is_end = true;

    let expected = [(4, -2), (3, -1), (4, -1), (5, -1)];
    game.move_left();
    assert_eq!(game.tetrimino().blocks(), expected);
    game.rotate();
    assert_eq!(game.tetrimino().blocks(), expected);
    game.hard_drop();
    assert_eq!(game.tetrimino().blocks(), expected);
    game.save();
    assert_eq!(game.field().as_vec(), vec![vec![""; 10]; 20]);
    game.hold();
    assert_eq!(game.tetrimino().blocks(), expected);
    assert!(game.held().is_none());
}

#[test]
fn hold_tetrimino() {
    let mut game = make_game();
    game.hold();
    assert_eq!(
        game.held().unwrap(),
        Tetrimino::new(Shape::T).move_to((0, 0))
    );
    assert_eq!(game.tetrimino(), &Tetrimino::new(Shape::L).move_to((3, -2)));
}

#[test]
fn reset_rotation_when_holding() {
    let mut game = make_game();
    game.rotate();
    game.hold();
    assert_eq!(
        game.held().unwrap(),
        Tetrimino::new(Shape::T).move_to((0, 0))
    );
}

#[test]
fn do_not_hold_twice_without_saving() {
    let mut game = make_game();
    game.hold();
    game.hold(); // actually doesn't hold
    assert_eq!(
        game.held().unwrap(),
        Tetrimino::new(Shape::T).move_to((0, 0))
    );
    assert_eq!(game.tetrimino(), &Tetrimino::new(Shape::L).move_to((3, -2)));
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
        Tetrimino::new(Shape::I).move_to((0, 0))
    );
    assert_eq!(game.tetrimino(), &Tetrimino::new(Shape::T).move_to((3, -2)));
}

#[test]
fn have_sum_of_removed_lines() {
    let mut game = make_game();
    game.field = Field::from_vec(
        [
            vec![vec![""; 10]; 26],
            vec![vec![
                "red", "red", "red", "", "", "", "red", "red", "red", "red",
            ]],
        ]
        .concat(),
    );

    assert_eq!(game.removed_lines(), 0);
    game.hard_drop();
    game.save();
    assert_eq!(game.removed_lines(), 1);
}
