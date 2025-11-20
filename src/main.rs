pub mod client;
pub mod base;
pub mod game;

fn main() {
    game::Game::default().start();
}

