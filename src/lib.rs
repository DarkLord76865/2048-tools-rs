//! **tools-2048** is a library that provides the core logic of the popular game 2048, along with a basic AI to play the game.
//!
//! Example usage:
//! ```rust
//! use tools_2048::Game2048;
//!
//! let mut game: Game2048 = Game2048::new();  // create a new game
//!
//! // make 10 moves
//! for _ in 0..10 {
//!     game.make_move(game.moves[0]);
//! }
//!
//! // make a move based on the AI's best move
//! game.make_move(game.find_best_move(10_000));
//!
//! assert!(game.score > 0);  // the score should be greater than 0
//! assert!(!game.is_game_over());  // the game should not be over yet
//!
//! ```


pub mod core_2048;
#[doc(inline)]
pub use core_2048::Game2048;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_full_game() {
        //! Test the AI's ability to play a full game

        let mut game = Game2048::new();
        loop {
            if game.make_move(game.find_best_move(10_000)) == 1 {
                break;
            }
        }

        assert!(game.is_game_over());
    }
}
