pub mod renderer;

use winit::event_loop::EventLoop;
use winit::event::{
    WindowEvent,
    ElementState,
};

use crate::game::networking::*;
use crate::game::physic::Direction;
use crate::client::renderer::GameRanderer;
use crate::game::map::Map;
use std::collections::HashMap;
use glium::backend::glutin::SimpleWindowBuilder;

use winit::keyboard::KeyCode;

use std::thread;
use std::sync::mpsc;


#[derive(Clone)]
enum InputEvents {
    Jump,
    Left,
    Right,

    Light,
    Heavy,
    Special,

    Switch(Option<u32>),

    Quit,
}

#[allow(dead_code)]
pub struct Client {
    name: String,
    password: String,

    addres: String,

    input: CharacterInput,
    game_pointer: Option<GameRanderer>,

    input_map: HashMap<KeyCode,InputEvents>,
    refresh_rate: u32,
}
impl Client {
    pub fn new(password: String, name: String, addres: String, refresh_rate: u32) -> Client {
        let mut input_map = HashMap::new();
        input_map.insert(KeyCode::KeyA,InputEvents::Left);
        input_map.insert(KeyCode::ArrowLeft,InputEvents::Left);

        input_map.insert(KeyCode::KeyD,InputEvents::Right);
        input_map.insert(KeyCode::ArrowRight,InputEvents::Right);

        input_map.insert(KeyCode::KeyW,InputEvents::Jump);
        input_map.insert(KeyCode::KeyC,InputEvents::Jump);

        input_map.insert(KeyCode::KeyX,InputEvents::Light);
        input_map.insert(KeyCode::KeyZ,InputEvents::Heavy);
        input_map.insert(KeyCode::ArrowDown,InputEvents::Special);

        input_map.insert(KeyCode::Escape,InputEvents::Quit);

        input_map.insert(KeyCode::Backspace,InputEvents::Switch(Option::None));
        input_map.insert(KeyCode::Digit0,InputEvents::Switch(Some(0)));
        input_map.insert(KeyCode::Digit1,InputEvents::Switch(Some(1)));
        Client {
            name,
            password,
            addres,
            input: CharacterInput::new(),
            game_pointer: Option::None,
            input_map,
            refresh_rate,
        }
    } 
    pub fn default() -> Client {
        let password = String::new();
        let name = String::from("User");
        let addres = String::from("localhost:3621");
        Self::new(password,name,addres,30)
    }
    pub fn start(&mut self) {
        let (map_trans, map_rec) = mpsc::channel::<Map>();
        let (input_trans, input_rec) = mpsc::channel::<winit::event::WindowEvent>();
        let addres = self.addres.clone();
        let input_map = self.input_map.clone();
        let (name,password) = (self.name.clone(),self.password.clone());
        let refresh_rate = self.refresh_rate.into();


        let _network = thread::spawn(move || {
            let mut input = CharacterInput::new();
            let mut left = false;
            let mut right = false;
            enum InputTypeEvent {
                Normal,
                CharacterSwitch(Option<u32>),
                Join,
                Quit,
            }
            let mut input_type = InputTypeEvent::Join;
            loop {
                for key_input in input_rec.try_iter() {
                    match key_input {
                        WindowEvent::KeyboardInput{event, ..} => {
                            if let winit::keyboard::PhysicalKey::Code(som)  = event.physical_key && let Some(opt) = input_map.get(&som){
                                match (opt,event.state) {
                                    (InputEvents::Jump, pressed) => {input.jump = ElementState::Pressed == pressed},

                                    (InputEvents::Left, ElementState::Pressed) => {input.dir = Some(Direction::Left);left = true;},
                                    (InputEvents::Right, ElementState::Pressed) => {input.dir = Some(Direction::Right);right = true;},

                                    (InputEvents::Left, ElementState::Released)  => {
                                        left = false;
                                        if right {
                                            input.dir = Some(Direction::Right);
                                        }else{
                                            input.dir = Option::None;
                                        }
                                    },
                                    (InputEvents::Right, ElementState::Released) => {
                                        right = false;
                                        if left {
                                            input.dir = Some(Direction::Left);
                                        }else{
                                            input.dir = Option::None;
                                        }
                                    },

                                    (InputEvents::Special, pressed) => {input.special = ElementState::Pressed == pressed},
                                    (InputEvents::Heavy, pressed) => {input.heavy_attack = ElementState::Pressed == pressed},
                                    (InputEvents::Light, pressed) => {input.light_attack = ElementState::Pressed == pressed},

                                    (InputEvents::Switch(targ), ElementState::Pressed) => {input_type = InputTypeEvent::CharacterSwitch(*targ)},
                                    (InputEvents::Quit,_) => {input_type = InputTypeEvent::Quit;}
                                    _ => {},
                                }
                            }
                        },
                        _ => {},
                    }
                }
                match input_type {
                    InputTypeEvent::Normal => {
                        let packet = GameControlPacket::new(password.clone(),name.clone(),input.clone()).to_string();
                        let response = reqwest::blocking::Client::new()
                            .put(format!("http://{addres}/map/"))
                            .body(packet)
                            .send().unwrap();
                        let body = response.text().unwrap();
                        if let Ok(map) = serde_json::from_str::<Map>(&body) {map_trans.send(map).unwrap();}
                    },
                    InputTypeEvent::CharacterSwitch(id) => {
                        let packet = CharacterSwitchRequest::new(password.clone(),name.clone(),id).to_string();
                        let response = reqwest::blocking::Client::new()
                            .put(format!("http://{addres}/character/"))
                            .body(packet)
                            .send().unwrap();
                        let body = response.text().unwrap();
                        if let Ok(map) = serde_json::from_str::<Map>(&body) {map_trans.send(map).unwrap();}
                    },
                    InputTypeEvent::Join => {
                        let packet = JoinRequest::new(password.clone(),name.clone()).to_string();
                        let response = reqwest::blocking::Client::new()
                            .post(format!("http://{addres}/map/"))
                            .body(packet)
                            .send().unwrap();
                        let body = response.text().unwrap();
                        if let Ok(map) = serde_json::from_str::<Map>(&body) {map_trans.send(map).unwrap();}
                    },
                    InputTypeEvent::Quit => panic!("Create better quit system"),
                }
                input_type = InputTypeEvent::Normal;
                
                thread::sleep(std::time::Duration::from_millis(refresh_rate));
            }
        });

        let event_loop = EventLoop::builder().build().expect("event loop building");
        let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
        let mut game_renderer = GameRanderer::new(map_rec,input_trans, window, display);
        let _ = event_loop.run_app(&mut game_renderer);
    }
}
