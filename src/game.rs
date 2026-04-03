pub mod character;
pub mod map;
pub mod networking;
pub mod physic;

use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use std::thread;
use crate::game::networking::*;
use crate::game::{
    character::Character,
    map::*,

};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

/// Object that holds information about the player to be able play or be automaticly kicked out of
/// the game after inactivity.
#[derive(Clone)]
pub struct Player {
    #[allow(unused)]
    id: String,
    last_ping: usize,
    #[allow(unused)]
    name: String,
    instance: Option<u32>,   
    pub input: CharacterInput,
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            id: uuid::Uuid::new_v4().to_string(),
            last_ping: 0,
            name,
            instance: None,
            input: CharacterInput::new(),
        }
    }
}
/// Main object for processing game logic, from user managment to physics 
pub struct Game {
    password: String,
    addres: String,

    players: Arc<Mutex<HashMap<String,Player>>>,
    characters: HashMap<u32,Character>,

    map: Arc<Mutex<Option<Map>>>,
    map_pool: HashMap<usize,MapInformation>,
    refresh_rate: f32,
}
impl Game {
    /// Loads deafult values for testing on a local server
    pub fn default() -> Game {
        Game {
            password: String::from(""),
            addres: String::from("127.0.0.1:3621"),
            characters: Character::load_all(Option::None, &String::from("./assets/")),
            players: Arc::new(HashMap::new().into()),
            map: Arc::new(Some(Map::test()).into()),
            map_pool: MapInformation::load_all(None, &String::from("./assets/")),
            refresh_rate: 60.0,
        }
    }
    /// Prepears Game object for start and loades all maps and characters in to memory
    pub fn new(password: String,addres: String,refresh_rate: f32,map_id: usize,assets: String) -> Game {
        Game {
            password,
            addres,
            characters: Character::load_all(Option::None,&assets),
            players: Arc::new(HashMap::new().into()),
            map: Arc::new(Some(Map::new(map_id)).into()),
            map_pool: MapInformation::load_all(None,&assets),
            refresh_rate,
        }
    }
    /// Enables networking and starts the physic simulation in a nother thread
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
        let delta = 1.0/self.refresh_rate;
        let frame_time = std::time::Duration::from_secs_f32(delta);
        loop {
            let next_frame = std::time::Instant::now();

            let players_input = Self::players_clone(&self.players);
            loop {
                if let Ok(ref mut map_opt) = self.map.try_lock() &&
                    let Some(map) = &mut **map_opt {
                        map.counter += 1;
                        map.set_inputs(players_input);
                        map.update(&self.characters,&self.map_pool,&delta);
                        break;

                }
            }

            thread::sleep(frame_time-(std::time::Instant::now() - next_frame));
        }
    }
    /// Clones the list of all players and returnes it.
    fn players_clone(players_ref: &Arc<Mutex<HashMap<String,Player>>>) -> HashMap<String,Player> {
        loop { if let Ok(ref mut players) = players_ref.try_lock(){
            let out = players.clone();
            let _ = players.iter_mut().map(|(_,p)|{p.last_ping+=1;p.input.reset();});
            return out;
        }};
    }
    /// If no player of that name exists, create that player.
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
    /// Updates player according to information in GameControlPacked sended by that player.
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
    /// Logic for switching characters between states.
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
    /// Returns current map state in a response object that is ready to be send.
    fn get_map_res(map_ref: &Arc<Mutex<Option<Map>>>) -> Response {
        loop { if let Ok(map) = map_ref.try_lock() {
            if let Some(out) = (*map).clone() {
                break Response::new(ResponseStatus::Ok,BodyType::Binary, &out.to_string());
            } else {
                break Response::status(ResponseStatus::Ok);
            }
        }}
    }
    /// Main function for handeling all network and managing what will be done with any receaved packed.
    /// Basic browser requests are redirected to the project github page
    /// All endpoints returns current state of the map if succesfull. 
    ///
    /// Get / -> redirects on "github.com/3ther-joyboy/Nebula"
    ///
    /// Get /map/ -> Current state of the map.
    ///
    /// Post /map/ -> New player joining.
    ///
    /// Put /map/ -> Waiting for character inputs
    ///
    /// Put /character/ -> Logic for chaning characters
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
