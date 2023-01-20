# tetris-core

This library provides the core functions of Tetris.

# How to use

1. Import `Game` and `BlockKind`.
2. Make a block kind selector.
   ```rust
   fn selector() -> BlockKind {
       // Return one of the kinds (probably you want to select randomly)
   }
   ```
3. Create a game.
   ```rust
   fn main() {
       // Create a game which has a 10x20 field and provides 3 next tetriminos
       let mut game = Game::new(10, 20, 3, selector);
       // Now you can move, rotate, etc. using `game`!
   }
   ```

# Documentation

```shell
cargo doc
```
