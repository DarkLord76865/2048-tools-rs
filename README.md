# tools-2048-rs
A Rust crate that provides the core logic of the popular game 2048, along with a basic AI to play the game.
Arbitrary board sizes are supported with the minimum being 4x4.

---

[**Documentation**](https://docs.rs/tools-2048/latest/tools_2048/ "docs.rs")

[**Crate**](https://crates.io/crates/tools-2048 "crates.io")

## Example
```rust
use tools_2048::{Game, GameMove, GameState, GameResult};

// create a new game
let mut game: Game<4> = Game::new().unwrap();

// make a move
game.make_move(GameMove::Left);
game.make_move(GameMove::Right);
game.make_move(GameMove::Up);
game.make_move(GameMove::Down);

// find the best move and make it
game.make_move(game.find_best_move(10_000).unwrap());

assert_eq!(game.state(), GameState::InProgress);  // the game should still be in progress
assert_eq!(game.result(), GameResult::Pending);  // the result shouldn't be decided yet
```

The AI is based on the [Monte Carlo algorithm](https://en.wikipedia.org/wiki/Monte_Carlo_algorithm), and uses parallelism to speed up the process.
At depth of 10 000, AI achieves 1024 tile ~100% of the time, 2048 tile ~96% of the time, and 4096 tile ~65% of the time.
