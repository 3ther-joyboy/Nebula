pub mod character;
pub mod map;
pub mod networking;
pub mod physic;

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
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

#[derive(Clone)]
#[allow(dead_code)]
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

    players: Arc<Mutex<HashMap<String,Player>>>,
    characters: HashMap<u32,Character>,

    map: Arc<Mutex<Option<Map>>>,
    refresh_rate: u32,
}
impl Game {
    pub fn default() -> Game {
        Game {
            password: String::from(""),
            addres: String::from("127.0.0.1:3621"),
            characters: Character::load_all(Option::None),
            players: Arc::new(HashMap::new().into()),
            map: Arc::new(Some(Map::test()).into()),
            refresh_rate: 30,
        }
    }
    pub fn new(password: String,addres: String,refresh_rate: u32) -> Game {
        Game {
            password,
            addres,
            characters: Character::load_all(Option::None),
            players: Arc::new(HashMap::new().into()),
            map: Arc::new(None.into()),
            refresh_rate,
        }
    }
    pub fn start(&mut self) {
        let listener = TcpListener::bind(self.addres.clone())
            .expect("Binding addres was unsucesfull");

        let (map_pointer,player_pointer) = (Arc::clone(&self.map),Arc::clone(&self.players));
        let password = self.password.clone();

        let _ = thread::spawn(move ||
            for stream_er in listener.incoming() {
                let stream = stream_er.unwrap();
                Self::handle_connection(stream,&map_pointer,&player_pointer,&password);
            }
        );
        loop {
            thread::sleep(Duration::from_millis(self.refresh_rate.into()));
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
    }
    fn players_clone(players_ref: &Arc<Mutex<HashMap<String,Player>>>) -> HashMap<String,Player> {
        loop { if let Ok(ref mut players) = players_ref.try_lock(){
            let out = players.clone();
            let _ = players.iter_mut().map(|(_,p)|{p.last_ping+=1;p.input.reset();});
            return out;
        }};
    }
    fn new_player(input: JoinRequest,players_ref: &Arc<Mutex<HashMap<String,Player>>>) -> Result<(),Response> {
        loop { if let Ok(ref mut players) = players_ref.try_lock(){
            if let Option::None = players.get(&input.player_name) {
                players.insert(input.player_name.clone(),Player::new(input.player_name));
                return Ok(());
            } else {
                return Err(Response::status(ResponseStatus::Forbiden));
            }
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
    fn player_switch_char(character: Option<u32>, player_name: String ,map_ref: &Arc<Mutex<Option<Map>>>, players_ref: &Arc<Mutex<HashMap<String,Player>>>) -> Result<(),Response> {
        let instance_id_op = loop { if let Ok(ref mut players) = players_ref.try_lock(){
            if let Some(player) = players.get_mut(&player_name) {
                player.last_ping = 0;
                break player.instance;
            } else {
                return Err(Response::status(ResponseStatus::Unauthorized));
            }
        }};
        let current_character_id = loop { if let Ok(ref mut map_op) = map_ref.try_lock() {
            if let Some(map) = &**map_op {
                if let Some(id) = instance_id_op && let Some(instance) = map.characters.get(&id) {
                    break Some(instance.character);
                } else {
                    break Option::None;
                }
            }else{return Err(Response::status(ResponseStatus::NotImplemented));}
        }};

        match (current_character_id,character) {
            (Option::None,Option::Some(new_id)) => {
                let new_object_id = loop { if let Ok(ref mut map_op) = map_ref.try_lock() {
                    if let Some(map) = &mut **map_op {
                        break map.new_istance(new_id);
                    }else{return Err(Response::status(ResponseStatus::Error));}
                }};
                loop { if let Ok(ref mut players) = players_ref.try_lock(){
                    if let Some(player) = players.get_mut(&player_name) {
                        player.instance = Some(new_object_id);
                        break;
                    } else {
                        return Err(Response::status(ResponseStatus::Error));
                    }
                }}
            },
            (Option::Some(_),Option::None) => {
                loop { if let Ok(ref mut map_op) = map_ref.try_lock() {
                    if let Some(map) = &mut **map_op && let Some(id) = instance_id_op {
                        map.characters.remove(&id);
                        break;
                    }else{return Err(Response::status(ResponseStatus::Error));}
                }}
                loop { if let Ok(ref mut players) = players_ref.try_lock(){
                    if let Some(player) = players.get_mut(&player_name) {
                        player.instance = Option::None;
                        break;
                    } else {
                        return Err(Response::status(ResponseStatus::Error));
                    }
                }}
            },
            (Option::Some(_),Option::Some(new_id)) => 
                loop { if let Ok(ref mut map_op) = map_ref.try_lock() {
                    if let Some(map) = &mut **map_op && let Some(instance_id) = instance_id_op && let Some(char_instance) = map.characters.get_mut(&instance_id) {
                        char_instance.character = new_id;
                        char_instance.reset();
                        break;
                    }else{return Err(Response::status(ResponseStatus::Error));}
                }},
            (Option::None,Option::None) => {},
        }
        Ok(())
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
                ("PUT","/character/") => 
                    if let Some(input) = get_responce::<CharacterSwitchRequest>(&mut stream,headers) {
                        if *password == input.server_password {
                            if let Err(out) = Self::player_switch_char(input.character,input.player_name,map_ref,players_ref) {
                                out
                            } else {Self::get_map_res(&map_ref)}
                        }else{Response::status(ResponseStatus::Unauthorized)}
                    }else{Response::status(ResponseStatus::ParseError)},
                ("POST","/map/") => 
                    if let Some(input) = get_responce::<JoinRequest>(&mut stream,headers) {
                        if *password == input.server_password {
                            if let Err(error_msg) = Self::new_player(input, &players_ref) {
                                error_msg
                            }else{Self::get_map_res(&map_ref)}
                        }else{Response::status(ResponseStatus::Unauthorized)}
                    }else{Response::status(ResponseStatus::ParseError)},
                ("PUT","/map/") => {
                    if let Some(input) = get_responce::<GameControlPacket>(&mut stream,headers){
                        if *password == input.server_password {
                            if let Err(error_msg) = Self::update_player(input, &players_ref) {
                                error_msg
                            }else{Self::get_map_res(&map_ref)}
                        }else{Response::status(ResponseStatus::Unauthorized)}
                    }else{Response::status(ResponseStatus::ParseError)}},
                ("GET","/map/") => Self::get_map_res(&map_ref),
                (_,_) => Response::status(ResponseStatus::None),
            };
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}
