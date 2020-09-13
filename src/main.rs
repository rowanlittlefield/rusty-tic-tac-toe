pub mod space;
pub mod board;
pub mod game;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.play();
}
