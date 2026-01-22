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
    base::Math,
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
    pub fn update_textures(&mut self, display: &mut Display<WindowSurface>) {
        for frame in &mut self.idling {frame.texture.load_texture(display)}
        for frame in &mut self.running {frame.texture.load_texture(display)}
        for frame in &mut self.rizing {frame.texture.load_texture(display)}
        for frame in &mut self.falling {frame.texture.load_texture(display)}
        for frame in &mut self.light_attack {frame.texture.load_texture(display)}
        for frame in &mut self.heavy_attack {frame.texture.load_texture(display)}
        for frame in &mut self.air_born_light_attack {frame.texture.load_texture(display)}
        for frame in &mut self.air_born_heavy_attack {frame.texture.load_texture(display)}
    }
    pub fn test() -> Animations {
        let empty_frame = AnimationFrame {
            hurt_sircles: vec![ColisionSircle {state: ColisionState::Vulnerable ,colision_shape: Sircle{radius:0.5,position: [0.0,0.5]}}],
            hit_sircles: Vec::new(),
            events: Vec::new(),
            texture: Texture::new(),
            hold: 128,
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
    pub fn load(char_id: u32,display_option: &mut Option<&mut Display<WindowSurface>>) -> Option<Character> {
        if char_id == 0 {
            let mut default = Self::default();
            if let Some(ref mut display) = display_option.as_mut() {
                default.animations.update_textures(display);
            }
            return Some(default);
        }
        let mut character_json = String::new();
        if let Ok(mut file) = File::open(format!("{0}{char_id}.json",Self::CHAR_PATH)) && let Ok(_) = file.read_to_string(&mut character_json){
            let char_result = serde_json::from_str::<Self>(&character_json);
            match char_result {
                Ok(mut output) => {
                    if let Some(ref mut display) = display_option.as_mut() {
                        output.animations.update_textures(display);
                    }
                    return Some(output);
                },
                Err(error) => {
                    println!("{error:?}");
                    return Option::None;
                }
            }
        }
        Option::None
    }
    pub fn load_all(display: Option<&mut Display<WindowSurface>>) -> HashMap<u32,Character> {
        let mut display: Option<&mut Display<WindowSurface>> = display;

        let mut out = HashMap::new();
        out.insert(0,Self::load(0,&mut display).expect("Loading a default character failed.."));
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
                    let Some(character) = Self::load(id_number,&mut display) { // is possible to load the character
                                                                               
                    out.insert(id_number,character);
                } 
            }
        }
        out
    }
    fn get_animations(&self, animation: &AnimationState) -> &Vec<AnimationFrame> {
        match animation {
            AnimationState::Idling => {&self.animations.idling},
            AnimationState::Running => {&self.animations.running},
            AnimationState::Jump => {&self.animations.jump},
            AnimationState::Rizing => {&self.animations.rizing},
            AnimationState::Falling => {&self.animations.falling},
            AnimationState::LightAttack => {&self.animations.light_attack},
            AnimationState::HeavyAttack => {&self.animations.heavy_attack},
            AnimationState::AirBornLightAttack => {&self.animations.air_born_light_attack},
            AnimationState::AirBornHeavyAttack => {&self.animations.air_born_heavy_attack},
        }
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
    animation_hold: u8,
    animation_frame: usize,

    #[serde(skip_serializing,skip_deserializing)]
    pub input: CharacterInput
}
impl CharacterInstance {
    pub fn new(character: u32,id: u32) -> CharacterInstance {
        CharacterInstance {
            character,
            object_id: id,

            position: [0.0,0.0],
            velocity: [0.0,0.0],
            direction: Direction::Right,
            airborn: true,
            vournable: ColisionState::Vulnerable,
            state: State::Actionable,
            damage: 0.0,

            animation: AnimationState::Idling,
            animation_hold: 0,
            animation_frame: 0,

            input: CharacterInput::new(),
        }
    }
    pub fn reset(&mut self) {
        self.animation = AnimationState::Idling;
        self.position = [0.0,0.0];
        self.velocity = [0.0,0.0];
    }
    fn check_colision(&self, col: &ColisionPlane, char_sheet: &Character) -> bool {
        let mut colider = char_sheet.colider.clone();
        colider.position = Math::add_vec(&self.position,&colider.position);

        if Math::distance(&colider.position, &col.position) > col.size/2.0 + colider.radius {
            return false;
        }
        match col.rotation {
            Orientation::Up | Orientation::Down => {
                (col.position[1] - colider.position[1]).abs() < colider.radius
            }
            Orientation::Left | Orientation::Right => {
                (col.position[0] - colider.position[0]).abs() < colider.radius
            }
        }
    }
    pub fn update(&mut self, char_sheet: &Character, map: &crate::game::MapInformation) {
        const DELTA: f32 = 1.0/40.0;
        const GRAVITY: f32 = 0.04;
        self.update_animation(char_sheet);

        self.velocity[1] -= DELTA * GRAVITY;
        self.airborn = true;


        // "b_XX" -> boolean
        let (mut b_up,mut b_down,mut b_right,mut b_left) = (
                self.velocity[1] < 0.0,
                self.velocity[1] > 0.0,
                self.velocity[0] < 0.0,
                self.velocity[0] > 0.0
            );
        for col in &map.statics {
            match col.rotation {
                Orientation::Down => 
                    if b_down && self.check_colision(col, &char_sheet) {
                        b_down = false;
                        self.velocity[1] = 0.0;
                    },
                Orientation::Up => 
                    if b_up && self.check_colision(col, &char_sheet) {
                        b_up = false;
                        self.velocity[1] = 0.0;
                        self.velocity[0] *= 0.8;
                        self.airborn = false;
                    },
                Orientation::Right => 
                    if b_right && self.check_colision(col, &char_sheet) {
                        b_right = false;
                        self.velocity[1] *= 0.8;
                        self.velocity[0] = 0.0;
                    },
                Orientation::Left => 
                    if b_left && self.check_colision(col, &char_sheet) {
                        b_left = false;
                        self.velocity[1] *= 0.8;
                        self.velocity[0] = 0.0;
                    },
            }
        }

        println!("{:?}",&self.velocity);
        let new_location = Math::add_vec(&self.position,&self.velocity);
        if Math::distance(&new_location,&self.position) < 0.0001 {
            self.velocity = [0.0,0.0];
        } else {
            self.position = new_location;
        }


        if self.input.jump {
            self.velocity[1] = 0.03;
        }
        match self.input.dir.clone() {
            Some(some) => {
                self.direction = some.clone();
                self.change_animation(AnimationState::Running);
                match some {
                    Direction::Left => {self.velocity[0] -= 0.005},
                    Direction::Right => {self.velocity[0] += 0.005},
                }
            },
            None => {
                self.change_animation(AnimationState::Idling);
            },
        }
    }
    fn change_animation(&mut self, anim: AnimationState) {
        if self.animation != anim {
            self.animation = anim;
            self.animation_hold = 0;
            self.animation_frame = 0;
        }
    }
    fn update_animation(&mut self,character: &Character) {
        let anim = &character.get_animations(&self.animation);

        if self.animation_hold >= anim[self.animation_frame].hold {
            if self.animation_frame + 1 >= anim.len() {
                self.animation_frame = 0;
            } else {
                self.animation_frame += 1;
            }
            self.animation_hold = 0;
        } else {
            self.animation_hold += 1
        }
    }
    pub fn draw(&self,display: &mut Display<WindowSurface>,frame_display: &mut glium::Frame,char_sheet: &HashMap<u32,Character>) {
        let character = char_sheet.get(&self.character).expect("Character that is trying to be rendered not found");
        let position = self.position;
        let frame = self.animation_frame;
        character.get_animations(&self.animation)[frame].texture.draw_on(display, frame_display, position,&self.direction);
    }
}
