//! # Super Tic Tic Toe
//! This library provides a core that implements the logic
//! for playing [Super Tic Tac Toe](https://en.wikipedia.org/wiki/Ultimate_tic-tac-toe).
//!
//! The rules of said game are explained in the Wikipedia entry, with **one exception**: since it wasn't specified in the Wikipedia article, the behavior for ties within small, traditional 3x3 tic tac toe boards will result in that board being unable to use. It will be "dead" or "locked"; nobody can use that board in their 3-in-a-row final win.
//!
//! ## Terminology
//! Because "board of boards" can get confusing on what you're referring
//! to, I'll define some terms:
//!
//! **Game:** the entire 9x9 super tic-tac-toe game or "large board"
//!
//! **Coordinates:** An (x, y) pair where x and y are integers between and including 0 to 2. It represents the location of a section (a square for a board, a board for a game).
//!
//! **Board:** a traditional 3x3 tic-tac-toe game or "small board"
//!
//! **Square:** a cell of a traditional tic-tac-toe board. It will either be empty or containing an `X`/`O`,
//!
//! **Square coordinates:** An (x, y) pair that, like a normal coordinate, represents the location of something. But unlike a regular coordinate, it represents the exact location of a specific square. X and Y will be integers between and including 0 to 8.
#![warn(missing_docs)]

use std::fmt::Display;
pub mod errors;
/// Represents a player (`X` or `O`)
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(missing_docs)]
pub enum Player {
    X,
    O,
}

/// Represents a the content of a smaller Tic Tac Toe board
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[allow(missing_docs)]
pub enum Square {
    #[default]
    Empty,
    Occupied(Player),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(missing_docs)]
pub enum GameState {
    Tie,
    Winner(Player),
    InProgress,
}

/// The size length of the board *and* the game. This should never change.
pub const BOARD_SIZE: usize = 3;
/// Represents the 3x3 traditional Tic Tac Toe board
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Board {
    /// Self explanatory. Public to allow implementations of display methods
    pub squares: [[Square; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn check_winner(&self, player: Player) -> bool {
        // Check rows, columns, and diagonals for a win within a 3x3 cell
        (0..BOARD_SIZE)
            .any(|i| (0..BOARD_SIZE).all(|j| self.squares[i][j] == Square::Occupied(player)))
            || (0..BOARD_SIZE)
                .any(|j| (0..BOARD_SIZE).all(|i| self.squares[i][j] == Square::Occupied(player)))
            || (0..BOARD_SIZE).all(|i| self.squares[i][i] == Square::Occupied(player))
            || (0..BOARD_SIZE).all(|i| self.squares[i][2 - i] == Square::Occupied(player))
    }
    /// Get the winner of the game, if any
    pub fn get_winner(&self) -> GameState {
        if self.check_winner(Player::O) {
            return GameState::Winner(Player::O);
        }
        if self.check_winner(Player::X) {
            return GameState::Winner(Player::X);
        }
        // All cells are full
        if self
            .squares
            .iter()
            .all(|cols| cols.iter().all(|game| matches!(game, Square::Occupied(_))))
        {
            return GameState::Tie;
        }
        GameState::InProgress
    }
}

/// Represents the 9x9 super Tic Tac Toe game. `X` starts
///
/// ## Example
///
/// ```
/// # use super_ttt::{Game, Player};
/// # fn main() {
/// let mut game = Game::new();
/// game.make_move(0, 0, 1, 1).unwrap();
/// game.make_move(1, 1, 0, 0).unwrap();
/// game.make_move(0, 0, 2, 2).unwrap();
/// game.make_move(2, 2, 0, 2).unwrap();
/// game.make_move(0, 2, 1, 0).unwrap();
/// match game.get_winner() {
///     super_ttt::GameState::Winner(player) => {
///         println!("Player {:?} wins!", player);
///     }
///     super_ttt::GameState::Tie => {
///         println!("It's a tie!");
///     }
///     super_ttt::GameState::InProgress => {
///         println!("The game is still in progress.");
///     }
/// }
/// # }
/// ```
///
/// This is essentially just a game state
/// with some relevant methods attached to it.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Game {
    /// Self explanatory. Public to allow implementations of display methods
    pub boards: [[Board; BOARD_SIZE]; BOARD_SIZE],
    /// The current player that will make the move when [`Game::make_move`] is called
    pub current_player: Player,
    /// The coordinates of the last move made by a player.
    pub last_move_cords: Option<(usize, usize)>,
}
impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┏━━━┳━━━┳━━━┓")?;
        for board_rows in 0..3 {
            for cell_row in 0..3 {
                write!(f, "┃")?;
                for board in 0..3 {
                    for cell_col in 0..3 {
                        let cell = self.boards[board_rows][board].squares[cell_row][cell_col];
                        let symbol = match cell {
                            Square::Empty => " ",
                            Square::Occupied(Player::X) => "X",
                            Square::Occupied(Player::O) => "O",
                        };
                        write!(f, "{}", symbol)?;
                    }
                    write!(f, "┃")?;
                }
                writeln!(f)?;
            }
            if board_rows == 2 {
                writeln!(f, "┗━━━┻━━━┻━━━┛")?;
            } else {
                writeln!(f, "┣━━━╋━━━╋━━━┫")?;
            }
        }
        Ok(())
    }
}
impl Game {
    /// Create a new game. Default starting player is [`Player::X`]
    pub fn new() -> Self {
        Game {
            boards: Default::default(),
            current_player: Player::X,
            last_move_cords: None,
        }
    }
    /// Make a move on the game. This method will also swap the [`Game::current_player`]
    pub fn make_move(
        &mut self,
        board_row: usize,
        board_col: usize,
        cell_row: usize,
        cell_col: usize,
    ) -> Result<Self, errors::InvalidMoveError> {
        // Check if the move is valid
        if self.boards[board_row][board_col].squares[cell_row][cell_col] != Square::Empty {
            return Err(errors::InvalidMoveError::CellAlreadyOccupied);
        }
        if let Some((x, y)) = self.last_move_cords {
            // X, Y is the coordinates of your opponent's last move
            // If these coordinates don't match with the coordinates
            // of the board that you want to put your piece in
            // and the board with matching coordinates of (X, Y) hasn't
            // finished, the move is illegal

            // For example, let's say my opponent played in (0, 0, 2, 2)
            // This means that they played in the top left board
            // but the bottom right cell within that board.
            // This means that my next move must be in the bottom right *board*
            // unless that board has already been finished (there was a win or tie)
            if (board_row, board_col) != (x, y)
                && matches!(self.boards[x][y].get_winner(), GameState::InProgress)
            {
                return Err(errors::InvalidMoveError::InvalidBoard);
            }
        }

        // Make the move
        self.boards[board_row][board_col].squares[cell_row][cell_col] =
            Square::Occupied(self.current_player);

        // Switch to the next player
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        self.last_move_cords = Some((cell_row, cell_col));

        Ok(*self)
    }

    /// Check if any of the boards has a winner
    fn check_winner(&self, player: Player) -> bool {
        // Columns
        (0..BOARD_SIZE).any(|i| (0..BOARD_SIZE).all(|j| self.boards[i][j].get_winner() == GameState::Winner(player)))
            || // Rows
            (0..BOARD_SIZE).any(|j| {
                (0..BOARD_SIZE).all(|i| self.boards[i][j].get_winner() == GameState::Winner(player))
            })
            ||  // y = -x Diagonals
            (0..BOARD_SIZE).all(|i| self.boards[i][i].get_winner() == GameState::Winner(player))
            || // y = x diagonals
            (0..BOARD_SIZE).all(|i| self.boards[i][2 - i].get_winner() == GameState::Winner(player))
    }
    /// Get the winner of the game, if any
    pub fn get_winner(&self) -> GameState {
        if self.check_winner(Player::O) {
            return GameState::Winner(Player::O);
        }
        if self.check_winner(Player::X) {
            return GameState::Winner(Player::X);
        }
        // All boards have been finished
        if self.boards.iter().all(|cols| {
            cols.iter()
                .all(|game| game.get_winner() != GameState::InProgress)
        }) {
            return GameState::Tie;
        }
        GameState::InProgress
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn game_struct_size() {
        assert_eq!(mem::size_of::<Game>(), 112);
    }
}
