use crate::{Game, Shape};

fn make_game() -> Game {
    let mut count = 0;
    let selector = Box::new(move || {
        count += 1;
        if count == 1 {
            Shape::T
        } else {
            Shape::L
        }
    });
    Game::new(10, 20, selector)
}

#[test]
fn create_10x20_field() {
    let game = make_game();
    let field = game.field();
    assert_eq!(field.width(), 10);
    assert_eq!(field.height(), 20);
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
    game.tetrimino = crate::Tetrimino::new(Shape::T).move_to((0, 0));

    game.move_left();
    assert_eq!(game.tetrimino().blocks(), [(1, 0), (0, 1), (1, 1), (2, 1)]);
}

#[test]
fn do_not_go_through_other_blocks() {
    // 7 is the height of the negative area
    let mut field = vec![vec![""; 4]; 7 + 1];
    field.push(vec!["red", "", "", ""]);
    let mut game = make_game();
    game.field = crate::Field::from_vec(field);
    game.tetrimino = crate::Tetrimino::new(Shape::T).move_to((1, 0));

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
    game.field = crate::Field::from_vec(vec![vec![""; 3]; 7 + 2]);
    game.tetrimino = crate::Tetrimino::new(Shape::T);

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
    game.field = crate::Field::from_vec(field_state);

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
fn end_when_saved_tetrimino_is_out_of_visible_area() {
    let mut game = make_game();
    game.field = crate::Field::from_vec(vec![vec![""; 10]; 7]);

    game.save();
    assert!(game.is_end());
}
