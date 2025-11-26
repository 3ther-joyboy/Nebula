pub mod client;
pub mod base;
pub mod game;

use std::thread;

fn main() {


    let server = thread::spawn(||game::Game::default().start());
    client::Client::default().start();
}

