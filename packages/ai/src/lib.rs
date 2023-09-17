use rand::{self, seq::SliceRandom};

use super_ttt::{errors, Game, GameState, Player, Square};
pub fn apply_move(
    (board_row, board_col, cell_row, cell_col): (usize, usize, usize, usize),
    mut game: Game,
) -> Result<Game, errors::InvalidMoveError> {
    game.make_move(board_row, board_col, cell_row, cell_col)
}
pub type Move = (usize, usize, usize, usize);
pub fn get_valid_moves(node: Game) -> Vec<Move> {
    let mut valid_moves = Vec::new();
    let mut row_range = 0..3;
    let mut col_range = 0..3;

    if let Some(last_move_cord) = node.last_move_cords {
        if node.boards[last_move_cord.0][last_move_cord.1].get_winner() == GameState::InProgress {
            row_range = last_move_cord.0..last_move_cord.0 + 1;
            col_range = last_move_cord.1..last_move_cord.1 + 1;
        }
    }

    for i in row_range {
        for j in col_range.clone() {
            for k in 0..3 {
                for l in 0..3 {
                    if node.boards[i][j].squares[k][l] == Square::Empty {
                        valid_moves.push((i, j, k, l));
                    }
                }
            }
        }
    }

    valid_moves
}
pub fn simulate_game(node: Game) -> (Vec<Game>, i8) {
    let current_node = node;
    let mut visited_nodes = vec![node];
    while current_node.get_winner() == GameState::InProgress {
        visited_nodes.push(
            apply_move(
                *get_valid_moves(current_node)
                    .choose(&mut rand::thread_rng())
                    .expect("No valid moves"),
                current_node,
            )
            .expect("Invalid move generated"),
        );
    }
    visited_nodes.push(current_node);
    (
        visited_nodes,
        match current_node.get_winner() {
            GameState::Tie => 0,
            GameState::Winner(player) => {
                if player == Player::X {
                    1
                } else {
                    -1
                }
            }
            GameState::InProgress => unreachable!(),
        },
    )
}
