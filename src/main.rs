pub mod client;
pub mod base;
pub mod game;

use std::thread;

#[macro_use]
extern crate glium;
fn main() {
    let _ = thread::spawn(||
            game::Game::new(String::from(""),String::from("localhost:3621"),40,1).start()
        );
    let _ = client::Client::new(String::from(""),String::from("Kuba"),String::from("localhost:3621"),40).start();
}

