mod utils;
mod game;

use game::Game;

fn main() {

    let mut game: Game = Game::new();
    game.start();

}
