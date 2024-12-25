mod game;
mod cell_value;
mod player;
use game::*;

fn main() {
    let mut game = Game::new();
    game.run_game();
}
