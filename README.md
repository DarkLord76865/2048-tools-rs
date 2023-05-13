# tools-2048-rs
A Rust crate that provides the core logic of the popular game 2048, along with a basic AI to play the game.
Arbitrary board sizes are supported, but the default, recommended one is 4x4.

---

[**Documentation**](https://docs.rs/tools-2048/latest/tools_2048/ "docs.rs")

[**Crate**](https://crates.io/crates/tools-2048 "crates.io")

## Example
```rust
use tools_2048::Game2048;

// create a new game
let mut game: Game2048 = Game2048::new();

// make 10 moves
for _ in 0..10 {
    // pick first valid move
    let random_move = *game.moves.iter().find(|&x| *x.1).unwrap().0;

    // make the move
    game.make_move(random_move).unwrap();
}

// make a move based on the AI's best move
game.make_move(game.find_best_move(10_000)).unwrap();

assert!(game.score > 0);  // the score should be greater than 0
assert!(!game.is_game_over());  // the game should not be over yet
assert!(!game.is_game_won());  // the game should not be won yet
```

The AI is based on the [Monte Carlo algorithm](https://en.wikipedia.org/wiki/Monte_Carlo_algorithm), and uses parallelism to speed up the process. At depth of 10 000, AI achieves 1024 tile 100% of the time, 2048 tile 96% of the time, and 4096 tile 65% of the time.
