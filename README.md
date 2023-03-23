# tools-2048-rs
A Rust crate that provides the core logic of the popular game 2048, along with a basic AI to play the game.

---

[**Documentation**](https://docs.rs/tools-2048/latest/tools_2048/ "docs.rs")

[**Crate**](https://crates.io/crates/tools-2048 "crates.io")

## Example
```rust
use tools_2048::Game2048;

let mut game: Game2048 = Game2048::new();  // create a new game

// make 10 moves
for _ in 0..10 {
    game.make_move(game.moves[0]);
}

// make a move based on the AI's best move
game.make_move(game.find_best_move(10_000));

assert!(game.score > 0);  // the score should be greater than 0
assert!(!game.is_game_over());  // the game should not be over yet
```

The AI is based on the [Monte Carlo algorithm](https://en.wikipedia.org/wiki/Monte_Carlo_algorithm), and uses parallelism to speed up the process.
