use rand::{prelude::thread_rng, Rng};
use tetris::{BlockKind, Game};

fn selector() -> BlockKind {
    BlockKind::all_as_array()[thread_rng().gen_range(0..7)]
}

fn main() {
    let mut game = Game::new(10, 20, 3, Box::new(selector));
    game.move_left();
    game.hard_drop();
    game.save();
    println!("{:?}", game.field());
    println!("{:?}", game.tetrimino());
}
