//! A module that contains the logic of the game 2048.

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::num::NonZeroUsize;
use std::thread::{available_parallelism, JoinHandle};
use std::thread;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::{SliceRandom, IteratorRandom};
use rand::thread_rng;


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
/// An enum that represents the moves that can be made in the game 2048.
pub enum Move2048 {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
/// An enum that represents the possible successful outcomes of a move in the game 2048.
pub enum Success2048 {
    Moved,
    Victory,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
/// An enum that represents the possible unsuccessful outcomes of a move in the game 2048.
pub enum Error2048 {
    InvalidMove,
    GameOver,
}


#[derive(Debug)]
/// A struct that represents the game 2048.
pub struct Game2048 {
    /// The board of the game.
    pub board: [[usize; 4]; 4],
    /// The score of the game.
    pub score: usize,
    /// The HashMap of the moves, where ```true``` means that the move is possible, and ```false``` means that the move is not possible.
    pub moves: HashMap<Move2048, bool>,
    /// Internal HashMap of the moves, where the value is a tuple of the new board state and the score.
    moves_values: HashMap<Move2048, ([[usize; 4]; 4], usize)>,
    /// Internal Vec used for storing empty tiles when spawning a new tile.
    empty_tiles: Vec<(usize, usize)>,
    /// Internal rng thread.
    rng_thrd: ThreadRng,
    /// Internal flag that indicates whether the game has been won.
    won: bool,
}

#[derive(Debug)]
/// A struct that represents the big game 2048 (bigger than 4x4).
pub struct BigGame2048 {
    /// The board of the game.
    pub board: Vec<Vec<usize>>,
    /// The score of the game.
    pub score: usize,
    /// The HashMap of the moves, where ```true``` means that the move is possible, and ```false``` means that the move is not possible.
    pub moves: HashMap<Move2048, bool>,
    /// Internal HashMap of the moves, where the value is a tuple of the new board state and the score.
    moves_values: HashMap<Move2048, (Vec<Vec<usize>>, usize)>,
    /// Internal Vec used for storing empty tiles when spawning a new tile.
    empty_tiles: Vec<(usize, usize)>,
    /// Internal Vec used for calculating moves (since board here is ```Vec<Vec<usize>>```, it isn't copy like in ```Game2048```, therefore it would be expensive to allocate new vector every time).
    working_board: Vec<Vec<usize>>,
    /// Internal rng thread.
    rng_thrd: ThreadRng,
    /// Internal flag that indicates whether the game has been won.
    won: bool,
}


impl Game2048 {
    pub fn new() -> Self {
        //! Creates a new game of 2048.

        let mut moves_map: HashMap<Move2048, bool> = HashMap::with_capacity(4);
        moves_map.insert(Move2048::Left, true);
        moves_map.insert(Move2048::Right, true);
        moves_map.insert(Move2048::Up, true);
        moves_map.insert(Move2048::Down, true);

        let mut moves_values_map: HashMap<Move2048, ([[usize; 4]; 4], usize)> = HashMap::with_capacity(4);
        moves_values_map.insert(Move2048::Left, ([[0; 4]; 4], 0));
        moves_values_map.insert(Move2048::Right, ([[0; 4]; 4], 0));
        moves_values_map.insert(Move2048::Up, ([[0; 4]; 4], 0));
        moves_values_map.insert(Move2048::Down, ([[0; 4]; 4], 0));

        let mut object = Self {
            board: [[0; 4]; 4],
            score: 0,
            moves: moves_map,
            moves_values: moves_values_map,
            empty_tiles: Vec::with_capacity(16),
            rng_thrd: thread_rng(),
            won: false,
        };
        object.new_tile();
        object.update_moves();
        object
    }

    pub fn from_existing(board: [[usize; 4]; 4]) -> Self {
        //! Creates a new game of 2048 from an existing board.
        //! # Arguments
        //! * ```board``` - The board of the game.

        let mut moves_map: HashMap<Move2048, bool> = HashMap::with_capacity(4);
        moves_map.insert(Move2048::Left, true);
        moves_map.insert(Move2048::Right, true);
        moves_map.insert(Move2048::Up, true);
        moves_map.insert(Move2048::Down, true);

        let mut moves_values_map: HashMap<Move2048, ([[usize; 4]; 4], usize)> = HashMap::with_capacity(4);
        moves_values_map.insert(Move2048::Left, ([[0; 4]; 4], 0));
        moves_values_map.insert(Move2048::Right, ([[0; 4]; 4], 0));
        moves_values_map.insert(Move2048::Up, ([[0; 4]; 4], 0));
        moves_values_map.insert(Move2048::Down, ([[0; 4]; 4], 0));

        let mut object = Self {
            board,
            score: 0,
            moves: moves_map,
            moves_values: moves_values_map,
            empty_tiles: Vec::with_capacity(16),
            rng_thrd: thread_rng(),
            won: false,
        };
        object.update_moves();
        object
    }

    pub fn is_game_over(&self) -> bool {
        //! A function that checks if the game is over.
        //! # Returns
        //! * ```true``` - The game is over.
        //! * ```false``` - The game is not over.

        for value in self.moves.values() {
            if *value {
                return false;
            }
        }
        true
    }

    pub fn is_game_won(&self) -> bool {
        //! A function that checks if the game is won.
        //! # Returns
        //! * ```true``` - The game is won.
        //! * ```false``` - The game is not won.

        for row in &self.board {
            for &tile in row {
                if tile >= 2048 {
                    return true;
                }
            }
        }
        false
    }

    pub fn make_move(&mut self, direction: Move2048) -> Result<Success2048, Error2048> {
        //! Makes a move in the game.
        //! # Arguments
        //! * ```direction``` - The direction of the move.
        //! # Returns
        //! * ```Ok(Success2048::Moved)```: The move was successful.
        //! * ```Ok(Success2048::Victory)```: The move was successful and the game was won.
        //! * ```Err(Error2048::InvalidMove)```: The move was invalid.
        //! * ```Err(Error2048::GameOver)```: The move was valid but the game is over.

        if self.moves[&direction] {
            for i in 0..self.moves_values[&direction].0.len() {
                for j in 0..self.moves_values[&direction].0[i].len() {
                    self.board[i][j] = self.moves_values[&direction].0[i][j];
                }
            }
            self.score += self.moves_values[&direction].1;
            self.new_tile();
            self.update_moves();
            if self.is_game_over() {
                Err(Error2048::GameOver)
            } else if !self.won && self.is_game_won() {
                self.won = true;
                Ok(Success2048::Victory)
            } else {
                Ok(Success2048::Moved)
            }
        } else {
            Err(Error2048::InvalidMove)
        }
    }

    fn new_tile(&mut self) {
        //! Adds a new tile to the board.
        //! Internal function.

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
        //! Internal function.

        let transpose = |board: &[[usize; 4]; 4]| -> [[usize; 4]; 4] {
            let mut new_board: [[usize; 4]; 4] = [[0; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    new_board[i][j] = board[j][i];
                }
            }
            new_board
        };

        let mut working_board_up: [[usize; 4]; 4] = transpose(&self.board);
        let mut working_board_down: [[usize; 4]; 4] = working_board_up;

        // up
        let mut score: usize = 0;
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
                    score += row[i];
                    row[i + 1] = 0;
                    row[(i + 1)..].rotate_left(1);
                }
            }
        }
        working_board_up = transpose(&working_board_up);
        if working_board_up != self.board {
            self.moves.insert(Move2048::Up, true);
            self.moves_values.get_mut(&Move2048::Up).unwrap().0 = working_board_up;
            self.moves_values.get_mut(&Move2048::Up).unwrap().1 = score;
        } else {
            self.moves.insert(Move2048::Up, false);
        }

        // down
        let mut score: usize = 0;
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
                    score += row[i];
                    row[i - 1] = 0;
                    row[..i].rotate_right(1);
                }
            }
        }
        working_board_down = transpose(&working_board_down);
        if working_board_down != self.board {
            self.moves.insert(Move2048::Down, true);
            self.moves_values.get_mut(&Move2048::Down).unwrap().0 = working_board_down;
            self.moves_values.get_mut(&Move2048::Down).unwrap().1 = score;
        } else {
            self.moves.insert(Move2048::Down, false);
        }

        // left
        let mut working_board_left: [[usize; 4]; 4] = self.board;
        let mut score: usize = 0;
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
                    score += row[i];
                    row[i + 1] = 0;
                    row[(i + 1)..].rotate_left(1);
                }
            }
        }
        if working_board_left != self.board {
            self.moves.insert(Move2048::Left, true);
            self.moves_values.get_mut(&Move2048::Left).unwrap().0 = working_board_left;
            self.moves_values.get_mut(&Move2048::Left).unwrap().1 = score;
        } else {
            self.moves.insert(Move2048::Left, false);
        }

        // right
        let mut working_board_right: [[usize; 4]; 4] = self.board;
        let mut score: usize = 0;
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
                    score += row[i];
                    row[i - 1] = 0;
                    row[..i].rotate_right(1);
                }
            }
        }
        if working_board_right != self.board {
            self.moves.insert(Move2048::Right, true);
            self.moves_values.get_mut(&Move2048::Right).unwrap().0 = working_board_right;
            self.moves_values.get_mut(&Move2048::Right).unwrap().1 = score;
        } else {
            self.moves.insert(Move2048::Right, false);
        }
    }

    pub fn find_best_move(&self, depth: usize) -> Move2048 {
        //! A function that finds the best move to make based on the current board and the depth of the search tree.
        //! Based on Monte Carlo algorithm (randomized guessing).
        //! Uses parallelism to speed up the process.
        //! # Arguments
        //! * ```depth``` - the depth of the search tree.
        //! # Returns
        //! * ```Move2048``` - the best move to make.

        let num_of_threads: usize = available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap()).get();
        let mut depth_per_thread: usize = depth / (self.moves.len() * num_of_threads);
        if depth_per_thread == 0 {
            depth_per_thread = 1;
        } else if depth_per_thread * (self.moves.len() * num_of_threads) != depth {
            depth_per_thread += 1;
        }
        let mut moves_values: HashMap<Move2048, usize> = HashMap::with_capacity(4);

        for move_ind in &self.moves {
            if !*move_ind.1 {continue;}
            let mut vec_of_threads: Vec<JoinHandle<usize>> = Vec::with_capacity(num_of_threads);
            let move_type: Move2048 = *move_ind.0;
            let current_board = self.board;

            for _ in 0..num_of_threads {
                vec_of_threads.push(thread::spawn(move || {
                    let mut thread_score: usize = 0;
                    let mut rng_thread: ThreadRng = thread_rng();

                    for _ in 0..depth_per_thread {
                        let mut new_board = Self::from_existing(current_board);

                        if let Err(err_type) = new_board.make_move(move_type) {
                            if err_type == Error2048::GameOver {break;}
                        } else {
                            loop {
                                if let Err(err_type) = new_board.make_move(new_board.moves.iter().filter(|&x| *x.1).map(|x| *x.0).choose(&mut rng_thread).unwrap()) {
                                    if err_type == Error2048::GameOver {break;}
                                }
                            }
                        }

                        thread_score += new_board.score;
                    }

                    thread_score
                }));
            }

            for thread_1 in vec_of_threads {
                moves_values.insert(move_type, *moves_values.get(&move_type).unwrap_or(&0) + thread_1.join().unwrap());
            }
        }
        *moves_values.iter().max_by_key(|&x| x.1).unwrap().0
    }
}

impl Default for Game2048 {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Game2048 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut max_val: usize = 0;
        for row in &self.board {
            for val in row {
                if *val > max_val {
                    max_val = *val;
                }
            }
        }
        let max_len: usize = max_val.to_string().chars().count() + 1;
        let mut output: String = String::from("Board:\n");
        for row in &self.board {
            for val in row {
                output += &format!("{:^width$}", val, width = max_len);
            }
            output += "\n";
        }
        output += &format!("Score: {}\n", self.score);
        write!(f, "{}", output)
    }
}


impl BigGame2048 {
    pub fn new(n: usize) -> Self {
        //! Creates a new big game of 2048.
        //! # Arguments
        //! * ```n```: The size of the board (```n```x```n```).
        //! # Panics
        //! * Panics if ```n``` < 5.

        assert!(n > 4, "The board size must be at least 5x5.");

        let mut moves_map: HashMap<Move2048, bool> = HashMap::with_capacity(4);
        moves_map.insert(Move2048::Left, true);
        moves_map.insert(Move2048::Right, true);
        moves_map.insert(Move2048::Up, true);
        moves_map.insert(Move2048::Down, true);

        let mut moves_values_map: HashMap<Move2048, (Vec<Vec<usize>>, usize)> = HashMap::with_capacity(4);
        moves_values_map.insert(Move2048::Left, (vec![vec![0; n]; n], 0));
        moves_values_map.insert(Move2048::Right, (vec![vec![0; n]; n], 0));
        moves_values_map.insert(Move2048::Up, (vec![vec![0; n]; n], 0));
        moves_values_map.insert(Move2048::Down, (vec![vec![0; n]; n], 0));

        let mut object: Self = Self {
            board: vec![vec![0; n]; n],
            score: 0,
            moves: moves_map,
            moves_values: moves_values_map,
            empty_tiles: Vec::with_capacity(16),
            working_board: vec![vec![0; n]; n],
            rng_thrd: thread_rng(),
            won: false,
        };
        object.new_tile();
        object.update_moves();
        object
    }

    pub fn from_existing(board: Vec<Vec<usize>>) -> Self {
        //! Creates a new big game of 2048 from an existing board.
        //! # Arguments
        //! * ```board```: The board to use.
        //! # Panics
        //! * Panics if ```board``` is not square.
        //! * Panics if ```board``` is smaller than 5x5.

        let n: usize = board.len();
        assert!(n > 4, "The board size must be at least 5x5.");
        for row in &board {
            assert_eq!(row.len(), n, "The board must be square.");
        }

        let mut moves_map: HashMap<Move2048, bool> = HashMap::with_capacity(4);
        moves_map.insert(Move2048::Left, true);
        moves_map.insert(Move2048::Right, true);
        moves_map.insert(Move2048::Up, true);
        moves_map.insert(Move2048::Down, true);

        let mut moves_values_map: HashMap<Move2048, (Vec<Vec<usize>>, usize)> = HashMap::with_capacity(4);
        moves_values_map.insert(Move2048::Left, (vec![vec![0; board.len()]; board.len()], 0));
        moves_values_map.insert(Move2048::Right, (vec![vec![0; board.len()]; board.len()], 0));
        moves_values_map.insert(Move2048::Up, (vec![vec![0; board.len()]; board.len()], 0));
        moves_values_map.insert(Move2048::Down, (vec![vec![0; board.len()]; board.len()], 0));

        let mut object = Self {
            board,
            score: 0,
            moves: moves_map,
            moves_values: moves_values_map,
            empty_tiles: Vec::with_capacity(16),
            working_board: vec![vec![0; n]; n],
            rng_thrd: thread_rng(),
            won: false,
        };
        object.update_moves();
        object
    }

    pub fn is_game_over(&self) -> bool {
        //! A function that checks if the game is over.
        //! # Returns
        //! * ```true``` - The game is over.
        //! * ```false``` - The game is not over.

        for value in self.moves.values() {
            if *value {
                return false;
            }
        }
        true
    }

    pub fn is_game_won(&self) -> bool {
        //! A function that checks if the game is won.
        //! # Returns
        //! * ```true``` - The game is won.
        //! * ```false``` - The game is not won.

        for row in &self.board {
            for &tile in row {
                if tile >= 2048 {
                    return true;
                }
            }
        }
        false
    }

    pub fn make_move(&mut self, direction: Move2048) -> Result<Success2048, Error2048> {
        //! Makes a move in the game.
        //! # Arguments
        //! * ```direction```: The direction to move in.
        //! # Returns
        //! * ```Ok(Success2048::Moved)```: The move was successful.
        //! * ```Ok(Success2048::Victory)```: The move was successful and the game was won.
        //! * ```Err(Error2048::InvalidMove)```: The move was invalid.
        //! * ```Err(Error2048::GameOver)```: The move was valid but the game is over.

        if self.moves[&direction] {
            for i in 0..self.moves_values[&direction].0.len() {
                for j in 0..self.moves_values[&direction].0[i].len() {
                    self.board[i][j] = self.moves_values[&direction].0[i][j];
                }
            }
            self.score += self.moves_values[&direction].1;
            self.new_tile();
            self.update_moves();
            if self.is_game_over() {
                Err(Error2048::GameOver)
            } else if !self.won && self.is_game_won() {
                self.won = true;
                Ok(Success2048::Victory)
            } else {
                Ok(Success2048::Moved)
            }
        } else {
            Err(Error2048::InvalidMove)
        }
    }

    fn new_tile(&mut self) {
        //! Adds a new tile to the board.
        //! Internal function.

        self.empty_tiles.clear();
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
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
        //! Internal function.

        // up
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.working_board[i][j] = self.board[j][i];
            }
        }
        let mut score: usize = 0;
        for row in &mut self.working_board {
            loop {
                let mut moved: bool = false;
                for i in 0..(row.len() - 1) {
                    if row[i] == 0 && row[i + 1] != 0 {
                        row.swap(i, i + 1);
                        moved = true;
                    }
                }
                if !moved {
                    break;
                }
            }
            for i in 0..(row.len() - 1) {
                if row[i] != 0 && row[i] == row[i + 1] {
                    row[i] *= 2;
                    score += row[i];
                    row[i + 1] = 0;
                    row[(i + 1)..].rotate_left(1);
                }
            }
        }
        let mut is_eq_transposed: bool = true;
        'outer_loop: for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] != self.working_board[j][i] {
                    is_eq_transposed = false;
                    break 'outer_loop;
                }
            }
        }
        if !is_eq_transposed {
            self.moves.insert(Move2048::Up, true);
            let board_ref = &mut self.moves_values.get_mut(&Move2048::Up).unwrap().0;
            for i in 0..board_ref.len() {
                for j in 0..board_ref[i].len() {
                    board_ref[i][j] = self.working_board[j][i];
                }
            }
            self.moves_values.get_mut(&Move2048::Up).unwrap().1 = score;
        } else {
            self.moves.insert(Move2048::Up, false);
        }

        // down
        let mut score: usize = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.working_board[i][j] = self.board[j][i];
            }
        }
        for row in &mut self.working_board {
            loop {
                let mut moved: bool = false;
                for i in 0..(row.len() - 1) {
                    if row[i] != 0 && row[i + 1] == 0 {
                        row.swap(i, i + 1);
                        moved = true;
                    }
                }
                if !moved {
                    break;
                }
            }
            for i in (1..row.len()).rev() {
                if row[i] != 0 && row[i] == row[i - 1] {
                    row[i] *= 2;
                    score += row[i];
                    row[i - 1] = 0;
                    row[..i].rotate_right(1);
                }
            }
        }
        let mut is_eq_transposed: bool = true;
        'outer_loop: for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] != self.working_board[j][i] {
                    is_eq_transposed = false;
                    break 'outer_loop;
                }
            }
        }
        if !is_eq_transposed {
            self.moves.insert(Move2048::Down, true);
            let board_ref = &mut self.moves_values.get_mut(&Move2048::Down).unwrap().0;
            for i in 0..board_ref.len() {
                for j in 0..board_ref[i].len() {
                    board_ref[i][j] = self.working_board[j][i];
                }
            }
            self.moves_values.get_mut(&Move2048::Down).unwrap().1 = score;
        } else {
            self.moves.insert(Move2048::Down, false);
        }

        // left
        let mut score: usize = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.working_board[i][j] = self.board[i][j];
            }
        }
        for row in &mut self.working_board {
            loop {
                let mut moved: bool = false;
                for i in 0..(row.len() - 1) {
                    if row[i] == 0 && row[i + 1] != 0 {
                        row.swap(i, i + 1);
                        moved = true;
                    }
                }
                if !moved {
                    break;
                }
            }
            for i in 0..(row.len() - 1) {
                if row[i] != 0 && row[i] == row[i + 1] {
                    row[i] *= 2;
                    score += row[i];
                    row[i + 1] = 0;
                    row[(i + 1)..].rotate_left(1);
                }
            }
        }
        let mut is_eq: bool = true;
        'outer_loop: for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] != self.working_board[i][j] {
                    is_eq = false;
                    break 'outer_loop;
                }
            }
        }
        if !is_eq {
            self.moves.insert(Move2048::Left, true);
            self.moves.insert(Move2048::Left, true);
            let board_ref = &mut self.moves_values.get_mut(&Move2048::Left).unwrap().0;
            for i in 0..board_ref.len() {
                for j in 0..board_ref[i].len() {
                    board_ref[i][j] = self.working_board[i][j];
                }
            }
            self.moves_values.get_mut(&Move2048::Left).unwrap().1 = score;
        } else {
            self.moves.insert(Move2048::Left, false);
        }

        // right
        let mut score: usize = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.working_board[i][j] = self.board[i][j];
            }
        }
        for row in &mut self.working_board {
            loop {
                let mut moved: bool = false;
                for i in 0..(row.len() - 1) {
                    if row[i] != 0 && row[i + 1] == 0 {
                        row.swap(i, i + 1);
                        moved = true;
                    }
                }
                if !moved {
                    break;
                }
            }
            for i in (1..row.len()).rev() {
                if row[i] != 0 && row[i] == row[i - 1] {
                    row[i] *= 2;
                    score += row[i];
                    row[i - 1] = 0;
                    row[..i].rotate_right(1);
                }
            }
        }
        let mut is_eq: bool = true;
        'outer_loop: for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] != self.working_board[i][j] {
                    is_eq = false;
                    break 'outer_loop;
                }
            }
        }
        if !is_eq {
            self.moves.insert(Move2048::Right, true);
            self.moves.insert(Move2048::Right, true);
            let board_ref = &mut self.moves_values.get_mut(&Move2048::Right).unwrap().0;
            for i in 0..board_ref.len() {
                for j in 0..board_ref[i].len() {
                    board_ref[i][j] = self.working_board[i][j];
                }
            }
            self.moves_values.get_mut(&Move2048::Right).unwrap().1 = score;
        } else {
            self.moves.insert(Move2048::Right, false);
        }
    }

    pub fn find_best_move(&self, depth: usize) -> Move2048 {
        //! A function that finds the best move to make based on the current board and the depth of the search tree.
        //! Based on Monte Carlo algorithm (randomized guessing).
        //! Uses parallelism to speed up the process.
        //! # Arguments
        //! * ```depth``` - the depth of the search tree.
        //! # Returns
        //! * ```Move2048``` - the best move to make.

        let num_of_threads: usize = available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap()).get();
        let possible_moves: usize = self.moves.values().filter(|&&x| x).count();
        let mut depth_per_thread: usize = depth / (possible_moves * num_of_threads);
        if depth_per_thread == 0 {
            depth_per_thread = 1;
        } else if depth_per_thread * (possible_moves * num_of_threads) != depth {
            depth_per_thread += 1;
        }
        let mut moves_values: HashMap<Move2048, usize> = HashMap::with_capacity(4);

        for move_ind in &self.moves {
            if !*move_ind.1 {continue;}
            let mut vec_of_threads: Vec<JoinHandle<usize>> = Vec::with_capacity(num_of_threads);
            let move_type: Move2048 = *move_ind.0;

            for _ in 0..num_of_threads {
                let cloned_board: Vec<Vec<usize>> = self.board.clone();
                vec_of_threads.push(thread::spawn(move || {
                    let mut thread_score: usize = 0;
                    let mut rng_thread: ThreadRng = thread_rng();

                    for _ in 0..depth_per_thread {
                        let mut new_board = Self::from_existing(cloned_board.clone());

                        if let Err(err_type) = new_board.make_move(move_type) {
                            if err_type == Error2048::GameOver {break;}
                        } else {
                            loop {
                                if let Err(err_type) = new_board.make_move(new_board.moves.iter().filter(|&x| *x.1).map(|x| *x.0).choose(&mut rng_thread).unwrap()) {
                                    if err_type == Error2048::GameOver {break;}
                                }
                            }
                        }
                        thread_score += new_board.score;
                    }
                    thread_score
                }));
            }
            for thread_1 in vec_of_threads {
                moves_values.insert(move_type, *moves_values.get(&move_type).unwrap_or(&0) + thread_1.join().unwrap());
            }
        }

        *moves_values.iter().max_by_key(|&x| x.1).unwrap().0
    }
}

impl Default for BigGame2048 {
    fn default() -> Self {
        Self::new(5)
    }
}

impl Display for BigGame2048 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut max_val: usize = 0;
        for row in &self.board {
            for val in row {
                if *val > max_val {
                    max_val = *val;
                }
            }
        }
        let max_len: usize = max_val.to_string().chars().count() + 1;
        let mut output: String = String::from("Board:\n");
        for row in &self.board {
            for val in row {
                output += &format!("{:^width$}", val, width = max_len);
            }
            output += "\n";
        }
        output += &format!("Score: {}\n", self.score);
        write!(f, "{}", output)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game2048() {
        //! Test the creation of a new game

        let game = Game2048::new();
        let game_from = Game2048::from_existing(game.board);

        assert_eq!(game.score, 0);
        assert_eq!(game_from.score, 0);
    }

    #[test]
    fn create_biggame2048() {
        //! Test the creation of a new big game

        let game = BigGame2048::new(5);
        let game_from = BigGame2048::from_existing(game.board.clone());

        assert_eq!(game.score, 0);
        assert_eq!(game_from.score, 0);
    }

    #[test]
    fn ai_game2048() {
        //! Test the AI's ability to play a game

        let mut game = Game2048::new();
        loop {
            if let Err(err_type) = game.make_move(game.find_best_move(1_000)) {
                if err_type == Error2048::GameOver {break;}
                else {panic!("Unexpected error: {:?}", err_type);}
            }
        }

        assert!(game.is_game_over());
    }

    #[test]
    fn ai_biggame2048() {
        //! Test the AI's ability to play a big game

        let mut game = BigGame2048::new(5);
        loop {
            match game.make_move(game.find_best_move(1_000)) {
                Ok(ok_type) => {
                    if ok_type == Success2048::Victory {break;}
                },
                Err(err_type) => {
                    if err_type == Error2048::GameOver {break;}
                    else {panic!("Unexpected error: {:?}", err_type);}
                }
            }
        }

        assert!(game.is_game_won() || game.is_game_over());
    }
}
