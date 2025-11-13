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

pub struct Player {
    id: String,
    last_ping: usize,
    name: String,
    instance: Option<u32>,   
    input: CharacterInput,
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
            // map: Arc::new(None.into()),
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
        println!("Threded");

        loop {
            thread::sleep(Duration::from_millis(50));
            loop {
                if let Ok(ref mut map_opt) = self.map.try_lock() &&
                    let Some(map) = &mut **map_opt {
                        map.counter += 1;
                        map.update(&self.characters);
                        break;

                }
            }
        }
        unreachable!();
    }
    fn get_map_res(map_ref: &Arc<Mutex<Option<Map>>>) -> Response {
        loop {
            if let Ok(map) = map_ref.try_lock() {
                if let Some(out) = (*map).clone() {
                    break Response::new(ResponseStatus::Ok,BodyType::JSON, &out.to_string());
                } else {
                    break Response::new(ResponseStatus::Ok,BodyType::JSON,"{}");
                }
            }
        }
    }
    fn handle_connection(mut stream: TcpStream,map_ref: &Arc<Mutex<Option<Map>>>,players_ref: &Arc<Mutex<HashMap<String,Player>>>, password: &String) {
        let mut buf_reader = BufReader::new(&stream).lines();
        let first_line = buf_reader.next().unwrap().unwrap();
        dbg!(&first_line);
        let rq_line: Vec<&str> = first_line.split_whitespace().collect();

        const GET: &&str = &"GET";
        const PUT: &&str = &"PUT";
        const POST: &&str = &"POST";
        let response = 
            if let (Some(one),Some(two)) = (rq_line.get(0),rq_line.get(1)) {
                match (one,two) {
                    (GET,&"/") => Response::new(ResponseStatus::Ok,BodyType::HTML, "<head><meta http-equiv=\"refresh\" content=\"0; url=https://github.com/3ther-joyboy/Nebula\" />"),
                    (POST,&"/map/") => {
                        let some = get_responce::<JoinRequest>(&mut buf_reader).unwrap();
                        todo!();
                    },
                    (PUT,&"/map/") => {
                        let input = get_responce::<GameControlPacket>(&mut buf_reader).unwrap();
                        let out = loop {
                            if let Ok(ref mut players) = players_ref.try_lock(){
                                if let Some(player) = players.get_mut(&input.player) {
                                   break Self::get_map_res(&map_ref);
                                } else {
                                    let err = String::from("something");
                                    break Response::new(ResponseStatus::Forbiden,BodyType::JSON,"");
                                }
                            }
                        };
                        out
                        
                    },
                    (GET,&"/map/") => Self::get_map_res(&map_ref),
                    (_,_) => Response::new(ResponseStatus::None,BodyType::JSON,""),
                }
            } else {
                Response::new(ResponseStatus::Error("REQUEST OR PATH NOT FOUND".to_string()),BodyType::JSON,"")
            };

        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}
