# tetris-core

This library provides the core functions of Tetris.

# How to use

1. Import `Game` and `Shape`.
2. Make a shape selector.
   ```rust
   fn selector() -> Shape {
       // Return one of the shapes (probably you want to select randomly)
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
