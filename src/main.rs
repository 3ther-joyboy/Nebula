pub mod client;
pub mod base;
pub mod game;

use std::thread;

#[macro_use]
extern crate glium;
fn main() {


    let _ = thread::spawn(||game::Game::default().start());
    let _ = client::Client::default().start();
}

