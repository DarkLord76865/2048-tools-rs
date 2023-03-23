//! A module that contains the logic of the game 2048.

use std::num::NonZeroUsize;
use std::thread::{available_parallelism, JoinHandle};
use std::thread;

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;


/// A struct that represents the game 2048.
pub struct Game2048 {
    /// The board of the game.
    pub board: [[u32; 4]; 4],
    /// The score of the game.
    pub score: u32,
    /// The moves that can be made. ```0```: left, ```1```: right, ```2```: up, ```3```: down.
    pub moves: Vec<usize>,
    moves_values: [Option<([[u32; 4]; 4], u32)>; 4],  // 0: left, 1: right, 2: up, 3: down, (move(new_board_state), score)
    empty_tiles: Vec<(usize, usize)>,
    rng_thrd: ThreadRng,
}

impl Game2048 {
    pub fn new() -> Self {
        //! Creates a new game of 2048.

        let mut object = Self {
            board: [[0; 4]; 4],
            score: 0,
            moves: Vec::with_capacity(4),
            moves_values: [Some(([[0; 4]; 4], 0)); 4],
            empty_tiles: Vec::with_capacity(16),
            rng_thrd: thread_rng(),
        };
        object.new_tile();
        object.update_moves();
        object
    }

    pub fn from_existing(board: [[u32; 4]; 4]) -> Self {
        //! Creates a new game of 2048 from an existing board.

        let mut object = Self {
            board,
            score: 0,
            moves: Vec::with_capacity(4),
            moves_values: [Some(([[0; 4]; 4], 0)); 4],
            empty_tiles: Vec::with_capacity(16),
            rng_thrd: thread_rng(),
        };
        object.update_moves();
        object
    }

    pub fn is_game_over(&self) -> bool {
        //! Returns true if the game is over.

        self.moves.is_empty()
    }

    pub fn make_move(&mut self, direction: usize) -> usize {  // 0: success, 1: game over, 2: invalid move
        //! Makes a move in the game.
        //! Returns ```0``` if the move was successful, ```1``` if the move was successful and now the game is over, and ```2``` if the move was invalid.

        if self.moves.contains(&direction) {
            let (new_board, score) = self.moves_values[direction].unwrap();
            self.board = new_board;
            self.score += score;
            self.new_tile();
            self.update_moves();
            if self.moves.is_empty() {
                1
            } else {
                0
            }
        } else {
            2
        }
    }

    fn new_tile(&mut self) {
        //! Adds a new tile to the board.

        self.empty_tiles.clear();
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 0 {
                    self.empty_tiles.push((i, j));
                }
            }
        }
        let location = self.empty_tiles.choose(&mut self.rng_thrd).unwrap();
        if self.rng_thrd.gen::<f64>() < 0.9 {
            self.board[location.0][location.1] = 2;
        } else {
            self.board[location.0][location.1] = 4;
        }
    }

    fn update_moves(&mut self) {
        //! Updates the moves that can be made.

        let transpose = |board: &[[u32; 4]; 4]| -> [[u32; 4]; 4] {
            let mut new_board: [[u32; 4]; 4] = [[0; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    new_board[i][j] = board[j][i];
                }
            }
            new_board
        };

        let mut score_left: u32 = 0;
        let mut score_right: u32 = 0;
        let mut score_up: u32 = 0;
        let mut score_down: u32 = 0;
        let mut working_board_up: [[u32; 4]; 4] = transpose(&self.board);
        let mut working_board_down: [[u32; 4]; 4] = working_board_up;
        let mut working_board_left: [[u32; 4]; 4] = self.board;
        let mut working_board_right: [[u32; 4]; 4] = self.board;

        // up
        for row in &mut working_board_up {
            loop {
                let mut moved: bool = false;
                for i in 0..3 {
                    if row[i] == 0 && row[i + 1] != 0 {
                        row.swap(i, i + 1);
                        moved = true;
                    }
                }
                if !moved {
                    break;
                }
            }
            for i in 0..3 {
                if row[i] != 0 && row[i] == row[i + 1] {
                    row[i] *= 2;
                    score_up += row[i];
                    row[i + 1] = 0;
                    row[(i + 1)..].rotate_left(1);
                }
            }
        }
        working_board_up = transpose(&working_board_up);
        if working_board_up != self.board {
            self.moves_values[2] = Some((working_board_up, score_up));
        } else {
            self.moves_values[2] = None;
        }

        // down
        for row in &mut working_board_down {
            loop {
                let mut moved: bool = false;
                for i in 0..3 {
                    if row[i] != 0 && row[i + 1] == 0 {
                        row.swap(i, i + 1);
                        moved = true;
                    }
                }
                if !moved {
                    break;
                }
            }
            for i in (1..4).rev() {
                if row[i] != 0 && row[i] == row[i - 1] {
                    row[i] *= 2;
                    score_down += row[i];
                    row[i - 1] = 0;
                    row[..i].rotate_right(1);
                }
            }
        }
        working_board_down = transpose(&working_board_down);
        if working_board_down != self.board {
            self.moves_values[3] = Some((working_board_down, score_down));
        } else {
            self.moves_values[3] = None;
        }

        // left
        for row in &mut working_board_left {
            loop {
                let mut moved: bool = false;
                for i in 0..3 {
                    if row[i] == 0 && row[i + 1] != 0 {
                        row.swap(i, i + 1);
                        moved = true;
                    }
                }
                if !moved {
                    break;
                }
            }
            for i in 0..3 {
                if row[i] != 0 && row[i] == row[i + 1] {
                    row[i] *= 2;
                    score_left += row[i];
                    row[i + 1] = 0;
                    row[(i + 1)..].rotate_left(1);
                }
            }
        }
        if working_board_left != self.board {
            self.moves_values[0] = Some((working_board_left, score_left));
        } else {
            self.moves_values[0] = None;
        }

        // right
        for row in &mut working_board_right {
            loop {
                let mut moved: bool = false;
                for i in 0..3 {
                    if row[i] != 0 && row[i + 1] == 0 {
                        row.swap(i, i + 1);
                        moved = true;
                    }
                }
                if !moved {
                    break;
                }
            }
            for i in (1..4).rev() {
                if row[i] != 0 && row[i] == row[i - 1] {
                    row[i] *= 2;
                    score_right += row[i];
                    row[i - 1] = 0;
                    row[..i].rotate_right(1);
                }
            }
        }
        if working_board_right != self.board {
            self.moves_values[1] = Some((working_board_right, score_right));
        } else {
            self.moves_values[1] = None;
        }

        // update public moves field
        self.moves.clear();
        for i in 0..4 {
            if self.moves_values[i].is_some() {
                self.moves.push(i);
            }
        }
    }

    pub fn find_best_move(&self, depth: usize) -> usize {
        //! Returns the best move (```0```: left, ```1```: right, ```2```: up, ```3```: down) to make based on the current board and the depth of the search tree.
        //! Based on Monte Carlo algorithm (randomized guessing).
        //! Uses parallelism to speed up the process.

        let num_of_threads: usize = available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap()).get();
        let mut depth_per_thread: usize = depth / (self.moves.len() * num_of_threads);
        if depth_per_thread == 0 {
            depth_per_thread = 1;
        } else if depth_per_thread * (self.moves.len() * num_of_threads) != depth {
            depth_per_thread += 1;
        }
        let mut moves_values: Vec<u32> = Vec::with_capacity(self.moves.len());

        for move_ind in &self.moves {
            let mut vec_of_threads: Vec<JoinHandle<u32>> = Vec::with_capacity(num_of_threads);
            let move_type = *move_ind;
            let current_board = self.board;

            for _ in 0..num_of_threads {
                vec_of_threads.push(thread::spawn(move || {
                    let mut thread_score: u32 = 0;
                    let mut rng_thread: ThreadRng = thread_rng();

                    for _ in 0..depth_per_thread {
                        let mut new_board = Self::from_existing(current_board);

                        if new_board.make_move(move_type) != 1 {
                            loop {
                                if new_board.make_move(*new_board.moves.choose(&mut rng_thread).unwrap()) == 1 {
                                    break;
                                }
                            }
                        }

                        thread_score += new_board.score;
                    }

                    thread_score
                }));
            }

            let mut sum: u32 = 0;
            for thread_1 in vec_of_threads {
                sum += thread_1.join().unwrap();
            }
            moves_values.push(sum);
        }

        self.moves[moves_values
            .iter()
            .position(|&x: &u32| {x == *moves_values.iter().max().unwrap()})
            .unwrap()]
    }
}

impl Default for Game2048 {
    fn default() -> Self {
        Self::new()
    }
}
