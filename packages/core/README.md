# Super Tic Tac Toe

This Rust library provides a core implementation of the logic for playing Super Tic Tac Toe. Super Tic Tac Toe is an extended version of the traditional Tic Tac Toe game, played on a 9x9 grid of smaller Tic Tac Toe boards. The rules of the game are explained in detail in the [Wikipedia entry](https://en.wikipedia.org/wiki/Ultimate_tic-tac-toe).

Docs are found [here](https://docs.rs/super-ttt)

## Installation

To use this library, add it to your `Cargo.toml` via

```
$ cargo add super-ttt
```

## Example

Here's an example that demonstrates how to use this library to play a game:

```rust
use super_ttt::{Game, Player};

fn main() {
    // Make moves and check for a winner
    let mut game = Game::new();
    game.make_move(0, 0, 1, 1).unwrap();
    game.make_move(1, 1, 0, 0).unwrap();
    game.make_move(0, 1, 2, 2).unwrap();
    game.make_move(2, 2, 0, 2).unwrap();
    game.make_move(0, 2, 1, 0).unwrap();

    match game.get_winner() {
        super_tic_tac_toe::GameState::Winner(player) => {
            println!("Player {:?} wins!", player);
        }
        super_tic_tac_toe::GameState::Tie => {
            println!("It's a tie!");
        }
        super_tic_tac_toe::GameState::InProgress => {
            println!("The game is still in progress.");
        }
    }
}

```

## Contributing

Contributions to this project are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License

This library is licensed under the MIT License. See the [LICENSE](LICENSE-MIT) file for more information.
