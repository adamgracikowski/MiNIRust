mod collatz;
mod collatz_game;
mod tuples;

use std::io;

use collatz_game::CollatzGame;
use tuples::tuple_operations;

fn main() -> io::Result<()> {
    let mut collatz = CollatzGame::default();
    match collatz.play()? {
        true => println!("Game exited due to an error."),
        false => println!("Game exited by the user."),
    }

    tuple_operations();

    Ok(())
}
