use super_ttt::{Game, GameState, Player, Square};
use text_io::read;
fn main() {
    let mut game = Game::new();
    loop {
        println!("┏━━━┳━━━┳━━━┓");
        for board_rows in 0..3 {
            for cell_row in 0..3 {
                print!("┃");
                for board in 0..3 {
                    for cell_col in 0..3 {
                        let cell = game.boards[board_rows][board].squares[cell_row][cell_col];
                        let symbol = match cell {
                            Square::Empty => " ",
                            Square::Occupied(Player::X) => "X",
                            Square::Occupied(Player::O) => "O",
                        };
                        print!("{}", symbol);
                    }
                    print!("┃");
                }
                println!();
            }
            if board_rows == 2 {
                println!("┗━━━┻━━━┻━━━┛");
            } else {
                println!("┣━━━╋━━━╋━━━┫");
            }
        }

        if let GameState::Winner(player) = game.get_winner() {
            println!("{:?} won", player);
            break;
        }
        let current_player = game.current_player;
        println!("Current player: {:?}", current_player);

        println!("Enter the row and column for your move (e.g., 0 0 1 1):");
        // let line = iterator.next().unwrap().unwrap();
        let bx: usize = read!();
        let by: usize = read!();
        let cx: usize = read!();
        let cy: usize = read!();
        match game.make_move(bx, by, cx, cy) {
            Ok(_) => {
                println!(
                    "{:?} moved in board ({}, {}) in cell ({}, {})",
                    current_player, bx, by, cx, cy
                )
            }
            Err(message) => {
                println!("{}", message)
            }
        }
    }
}
