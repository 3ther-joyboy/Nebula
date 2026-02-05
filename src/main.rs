pub mod client;
pub mod base;
pub mod game;

use std::thread;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server addres and port.
    #[arg(short, long, default_value_t = String::from("localhost:3621"))]
    addres: String,
    /// Server acces password
    #[arg(short, long, default_value_t = String::new())]
    password: String,
    /// Refresh rate (tps/fps)
    #[arg(short, long, default_value_t = 60.0)]
    time: f32,
    /// Starting map id
    #[arg(short, long, default_value_t = 1)]
    map: usize,

    /// Start without a server
    #[arg(short, long, default_value_t = false)]
    no_server: bool,

    /// Client name
    #[arg(short, long)]
    client: Option<String>,
}


#[macro_use]
extern crate glium;
fn main() {
    let args = Args::parse();

    let password = args.password.clone();
    let time = args.time;
    let addres = args.addres.clone();
    let opt_client = args.client.clone();

    let opt_server = if !args.no_server {
            Some( thread::spawn(move || {
                    game::Game::new(args.password,args.addres,args.time,args.map).start();
                }))
        } else {
            None
        };

    if let Some(client) = opt_client {
        let mut client = client::Client::new(password,client,addres,time);
        client.start()
    } else if let Some(server) = opt_server {
        let _ = server.join().unwrap();
    }

}

