pub mod renderer;

use crate::game::networking::*;
use gilrs::{Gilrs, Button, Event};

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

pub struct Client {
    name: String,
    password: String,

    addres: String,
    input: CharacterInput,
    // input_map: HashMap<input_event,>
}
impl Client {
    pub fn new(password: String, name: String, addres: String) -> Client {
        Client {
            name,
            password,
            addres,
            input: CharacterInput::new(),
        }
    } 
    pub fn default() -> Client {
        let password = String::new();
        let name = String::from("User");
        let addres = String::from("127.0.0.1:3621");
        Self::new(password,name,addres)
    }
    pub fn start(&mut self) {


        let mut gilrs = Gilrs::new().unwrap();

        // Iterate over all connected gamepads
        for (_id, gamepad) in gilrs.gamepads() {
            println!("{} is {:?}", gamepad.name(), gamepad.power_info());
        }
        let mut active_gamepad = None;


        loop {
                        // Examine new events
            while let Some(Event { id, event, time,.. }) = gilrs.next_event() {
                println!("{:?} New event from {}: {:?}", time, id, event);
                active_gamepad = Some(id);
            }
        }
    }
}
