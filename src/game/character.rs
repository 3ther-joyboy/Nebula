use std::{
    collections::HashMap,
    fs::{
        self,
        File,
    },
    io::Read,
};


use glium::{
    glutin::surface::WindowSurface,
    Display,
};

use crate::game::CharacterInput;
use serde::{
    Serialize,
    Deserialize,
};
use crate::{
    base::sircle::*,
    game::physic::*,
    client::renderer::*,
};


#[derive(Serialize, Deserialize, Clone)]
pub struct Animations {
    idling: Vec<AnimationFrame>,
    running: Vec<AnimationFrame>,
    jump: Vec<AnimationFrame>,
    rizing: Vec<AnimationFrame>,
    falling: Vec<AnimationFrame>,
    light_attack: Vec<AnimationFrame>,
    heavy_attack: Vec<AnimationFrame>,
    air_born_light_attack: Vec<AnimationFrame>,
    air_born_heavy_attack: Vec<AnimationFrame>,
}
impl Animations {
    pub fn test() -> Animations {
        let empty_frame = AnimationFrame {
            hurt_sircles: vec![ColisionSircle {state: ColisionState::Vulnerable ,colision_shape: Sircle{radius:0.5,position: [0.0,0.5]}}],
            hit_sircles: Vec::new(),
            events: Vec::new(),
            texture: Texture::new(),
        };
        Animations {
            idling: vec![empty_frame],
            running: Vec::new(),
            jump: Vec::new(),
            rizing: Vec::new(),
            falling: Vec::new(),
            light_attack: Vec::new(),
            heavy_attack: Vec::new(),
            air_born_light_attack: Vec::new(),
            air_born_heavy_attack: Vec::new(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    id: usize,
    name: String,
    weight: f32,
    air_jump_count: u32,
    aceleration: f32,
    max_speed: f32,
    colider: Sircle,

    animations: Animations,
}
impl Character {
    fn default() -> Character {
        Character {
            id: 0,
            name: String::new(),
            weight: 1.0,
            air_jump_count: 1,
            aceleration: 10.0,
            max_speed: 5.0,
            colider: Sircle {radius: 0.3, position: [0.0,0.3]},
            animations: Animations::test(),
        }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    const CHAR_PATH: &str = "./assets/characters/";
    pub fn load(char_id: u32) -> Option<Character> {
        if char_id == 0 {
            return Some(Self::default());
        }
        let mut character_json = String::new();
        if let Ok(mut file) = File::open(format!("{0}{char_id}.json",Self::CHAR_PATH)) && let Ok(_) = file.read_to_string(&mut character_json){
            return if let Ok(output) = serde_json::from_str::<Self>(&character_json) {
                Some(output)
            } else {
                Option::None
            }
        }
        Option::None
    }
    pub fn load_all() -> HashMap<u32,Character> {
        let mut out = HashMap::new();
        out.insert(0,Self::default());
        if let Ok(items_directory) = fs::read_dir(Self::CHAR_PATH) {
            for character_files in items_directory {
                if  let Ok(something) = character_files &&
                    let Ok(file_type) = something.file_type() && // has a file type
                    file_type.is_file() && // is a file (not dir or linked)
                    let Ok(name) = something.file_name().into_string() && // is possible to convert
                                                                          // name in to regurall ascii
                    name.len() > ".json".len() && // has more characters then .json thingie
                    name[name.len()-5..] == *".json" && // is last few chars ".json"
                    let Ok(id_number) = name[..name.len()-5].parse::<u32>() && // parse the the
                                                                               // name in to a number
                    let Some(character) = Self::load(id_number) { // is possible to load the character

                    out.insert(id_number,character);
                } 
            }
        }
        out
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterInstance {
    pub character: u32,
    object_id: u32,

    pub position: [f32;2],
    velocity: [f32;2],
    direction: Direction,
    airborn: bool,

    vournable: ColisionState,
    state: State,
    damage: f32,

    animation: AnimationState,

    #[serde(skip_serializing,skip_deserializing)]
    pub input: CharacterInput
}
impl CharacterInstance {
    pub fn new(character: u32,_id: u32) -> CharacterInstance {
        CharacterInstance {
            character,
            object_id: 0,

            position: [0.0,0.0],
            velocity: [0.0,0.0],
            direction: Direction::Left,
            airborn: true,
            vournable: ColisionState::Vulnerable,
            state: State::Actionable,
            damage: 0.0,
            animation: AnimationState::Idling(0),

            input: CharacterInput::new(),
        }
    }
    pub fn update(&mut self, _char_sheet: &Character) {
        match self.input.dir.clone() {
            Some(Direction::Left) => {self.position[0] -= 1.0},
            Some(Direction::Right) => {self.position[0] += 1.0},
            None => {},
        }
    }
    pub fn draw(&self,display: &mut Display<WindowSurface>,frame_display: &mut glium::Frame,char_sheet: &HashMap<u32,Character>) {
        let character = char_sheet.get(&self.character).expect("Character that is trying to be rendered not found");
        let position = self.position;

        match self.animation {
            AnimationState::Idling(frame) => character.animations.idling[frame].texture.draw_on(display, frame_display, position),
            AnimationState::Running(frame) => character.animations.running[frame].texture.draw_on(display, frame_display, position),
            AnimationState::Jump(frame) => character.animations.jump[frame].texture.draw_on(display, frame_display, position),
            AnimationState::Rizing(frame) => character.animations.rizing[frame].texture.draw_on(display, frame_display, position),
            AnimationState::Falling(frame) => character.animations.falling[frame].texture.draw_on(display, frame_display, position),
            AnimationState::LightAttack(frame) => character.animations.light_attack[frame].texture.draw_on(display, frame_display, position),
            AnimationState::HeavyAttack(frame) => character.animations.heavy_attack[frame].texture.draw_on(display, frame_display, position),
            AnimationState::AirBornLightAttack(frame) => character.animations.air_born_light_attack[frame].texture.draw_on(display, frame_display, position),
            AnimationState::AirBornHeavyAttack(frame) => character.animations.air_born_heavy_attack[frame].texture.draw_on(display, frame_display, position),
        }
    }
    
    //     let anim_tree = &char_sheet.get(&self.character)
    //         .expect("Character by ID: {char_sheet_id} not found.\nTrying to render unknow character.")
    //         .animations;
    //     let display, frame, position = offset + self.position.clone();
    // }
}
