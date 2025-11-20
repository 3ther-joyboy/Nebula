pub mod character;
pub mod map;
pub mod physic;
mod networking;

use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use std::collections::HashMap;
use std::thread;
use crate::game::networking::*;
use uuid::Uuid;
use crate::game::{
    character::Character,
    map::*,

};
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

#[derive(Clone)]
pub struct Player {
    id: String,
    last_ping: usize,
    name: String,
    instance: Option<u32>,   
    pub input: CharacterInput,
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            id: Uuid::new_v4().to_string(),
            last_ping: 0,
            name,
            instance: None,
            input: CharacterInput::new(),
        }
    }
}
pub struct Game {
    password: String,
    addres: String,

    characters: HashMap<u32,Character>,
    players: Arc<Mutex<HashMap<String,Player>>>,

    map: Arc<Mutex<Option<Map>>>,
}
impl Game {
    pub fn default() -> Game {
        let mut characters: HashMap<u32,Character> = HashMap::new();
        characters.insert(0,Character::load(0));
        Game {
            password: String::from(""),
            addres: String::from("127.0.0.1:3621"),
            characters,
            players: Arc::new(HashMap::new().into()),
            // map: Arc::new(None.into()), todo!();
            map: Arc::new(Some(Map::test()).into()),
        }
    }
    pub fn new(password: String,addres: String) -> Game {
        Game {
            password,
            addres,
            characters: Character::load_all(),
            players: Arc::new(HashMap::new().into()),
            map: Arc::new(None.into()),
        }
    }
    pub fn start(&mut self) {
        let listener = TcpListener::bind(self.addres.clone())
            .expect("Binding addres was unsucesfull");

        let (map_pointer,player_pointer) = (Arc::clone(&self.map),Arc::clone(&self.players));
        let password = self.password.clone();

        let network = thread::spawn(move ||
            for stream_er in listener.incoming() {
                let stream = stream_er.unwrap();
                Self::handle_connection(stream,&map_pointer,&player_pointer,&password);
            }
        );
        loop {
            thread::sleep(Duration::from_millis(50));
            let players_input = Self::players_clone(&self.players);
            loop {
                if let Ok(ref mut map_opt) = self.map.try_lock() &&
                    let Some(map) = &mut **map_opt {
                        map.counter += 1;
                        map.set_inputs(players_input);
                        map.update(&self.characters);
                        break;

                }
            }
        }
        unreachable!();
    }
    fn players_clone(players_ref: &Arc<Mutex<HashMap<String,Player>>>) -> HashMap<String,Player> {
        loop { if let Ok(ref mut players) = players_ref.try_lock(){
            let out = players.clone();
            let _ = players.iter_mut().map(|(_,mut p)|{p.last_ping+=1;p.input.reset();});
            return out;
        }};
    }
    fn update_player(input: GameControlPacket,players_ref: &Arc<Mutex<HashMap<String,Player>>>) -> Result<(),Response> {
        loop { if let Ok(ref mut players) = players_ref.try_lock(){
            if let Some(player) = players.get_mut(&input.player) {
                player.input = input.input;
                player.last_ping = 0;
                return Ok(());
            } else {
                return Err(Response::status(ResponseStatus::None));
            }
        }};
    }
    fn get_map_res(map_ref: &Arc<Mutex<Option<Map>>>) -> Response {
        loop { if let Ok(map) = map_ref.try_lock() {
            if let Some(out) = (*map).clone() {
                break Response::new(ResponseStatus::Ok,BodyType::JSON, &out.to_string());
            } else {
                break Response::status(ResponseStatus::Ok);
            }
        }}
    }
    fn handle_connection(mut stream: TcpStream,map_ref: &Arc<Mutex<Option<Map>>>,players_ref: &Arc<Mutex<HashMap<String,Player>>>, password: &String) {
        let headers = Headers::new(&mut stream);

        let matching = (headers.request_type.as_str(),headers.path.as_str());
        let response = 
            match matching {
                ("GET","/") => Response::new(ResponseStatus::Ok,BodyType::HTML, "<head><meta http-equiv=\"refresh\" content=\"0; url=https://github.com/3ther-joyboy/Nebula\" />"),
                ("POST","/map/") => {
                    let some = get_responce::<JoinRequest>(&mut stream,headers).unwrap();
                    todo!();
                },
                ("PUT","/map/") => {
                    if let Some(input) = get_responce::<GameControlPacket>(&mut stream,headers){
                        if *password == input.server_password {
                            if let Err(error_msg) = Self::update_player(input, &players_ref) {
                                error_msg
                            } else {
                                Self::get_map_res(&map_ref)
                            }
                        } else {
                            Response::status(ResponseStatus::Forbiden)
                        }
                    } else {
                        Response::status(ResponseStatus::ParseError)
                    }

                },
                ("GET","/map/") => Self::get_map_res(&map_ref),
                (_,_) => Response::new(ResponseStatus::None,BodyType::JSON,""),
            };
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}
