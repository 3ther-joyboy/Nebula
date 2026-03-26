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
    #[serde(default)]
    hurt: Vec<AnimationFrame>,
    idling: Vec<AnimationFrame>,
    running: Vec<AnimationFrame>,
    rizing: Vec<AnimationFrame>,
    falling: Vec<AnimationFrame>,
    light_attack: Vec<AnimationFrame>,
    heavy_attack: Vec<AnimationFrame>,
    air_born_light_attack: Vec<AnimationFrame>,
    air_born_heavy_attack: Vec<AnimationFrame>,
}
impl Animations {
    pub fn update_textures(&mut self, display: &mut Display<WindowSurface>) {
        for frame in &mut self.hurt {frame.texture.load_texture(display)}

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
            hurt: Vec::new(),
            running: Vec::new(),
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
    jump: f32,
    air_jump_count: u8,
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
            jump: 0.05,
            aceleration: 10.0,
            max_speed: 5.0,
            colider: Sircle {radius: 0.3, position: [0.0,0.3]},
            animations: Animations::test(),
        }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    const CHAR_PATH: &str = "characters/";
    pub fn load(char_id: u32,display_option: &mut Option<&mut Display<WindowSurface>>, assets: &String) -> Option<Character> {
        if char_id == 0 {
            let mut default = Self::default();
            if let Some(ref mut display) = display_option.as_mut() {
                default.animations.update_textures(display);
            }
            return Some(default);
        }
        let mut character_json = String::new();
        let path = format!("{assets}{0}{char_id}.json",Self::CHAR_PATH);
        if let Ok(mut file) = File::open(path) && let Ok(_) = file.read_to_string(&mut character_json){
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
    pub fn load_all(display: Option<&mut Display<WindowSurface>>, assets: &String) -> HashMap<u32,Character> {
        let mut display: Option<&mut Display<WindowSurface>> = display;

        let mut out = HashMap::new();
        out.insert(0,Self::load(0,&mut display, assets).expect("Loading a default character failed.."));
        if let Ok(items_directory) = fs::read_dir(format!("{assets}{0}",Self::CHAR_PATH)) {
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
                    let Some(character) = Self::load(id_number,&mut display, assets) { // is possible to load the character
                                                                               
                    out.insert(id_number,character);
                } 
            }
        }
        out
    }
    pub fn get_animations(&self, animation: &AnimationState) -> &Vec<AnimationFrame> {
        match animation {
            AnimationState::Damadged => {&self.animations.hurt},
            AnimationState::Idling => {&self.animations.idling},
            AnimationState::Running => {&self.animations.running},
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
    air_jump: u8,
    air_action: u8,

    vournable: ColisionState,
    state: State,
    damage: f32,

    animation: AnimationState,
    animation_hold: u8,
    animation_frame: usize,

    #[serde(skip_serializing,skip_deserializing)]
    pub input: CharacterInput,
    #[serde(skip_serializing,skip_deserializing)]
    pub last_input: CharacterInput
}
impl CharacterInstance {
    const AIR_ACTION_DEFAULT: u8 = 2;
    pub fn get_animation_frame(&self) -> usize {
        self.animation_frame
    }
    pub fn get_animatin(&self) -> &AnimationState {
        &self.animation
    }
    pub fn new(character: u32,id: u32) -> CharacterInstance {
        CharacterInstance {
            character,
            object_id: id,

            position: [0.0,0.0],
            velocity: [0.0,0.0],
            direction: Direction::Right,
            airborn: true,
            air_jump: 0,
            air_action: Self::AIR_ACTION_DEFAULT,
            vournable: ColisionState::Vulnerable,
            state: State::Actionable,
            damage: 0.0,

            animation: AnimationState::Idling,
            animation_hold: 0,
            animation_frame: 0,

            input: CharacterInput::new(),
            last_input: CharacterInput::new(),
        }
    }
    pub fn reset(&mut self) {
        *self = Self::new(self.character, self.object_id);
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
    pub fn hit_registration(&mut self,enemy: &Self, char_sheets: &HashMap<u32,Character>) {
        let (_,hurt_sircles) = self.get_hitboxes(char_sheets);
        let (hit_sircles,_) = enemy.get_hitboxes(char_sheets);

        if let crate::game::physic::State::HitStun(_,(id,frame)) = self.state.clone() &&
            id == enemy.object_id && frame == enemy.animation_frame {
            return;
        }

        for hit_sircle in &hit_sircles {
            for hurt_sircle in &hurt_sircles {
                if hurt_sircle.colision_shape.overlap(&self.position,&hit_sircle.colision_shape,&enemy.position) {
                    for event in &hit_sircle.impact_events {
                        if let Some(character) = char_sheets.get(&self.character) {
                            self.apply_frame_event(&event,enemy,&character);
                        } else {
                            println!("Character with id \"{0}\" not found",self.character);
                        }
                    }
                    break;
                }
            }
        }
    }
    fn jump_just_pressed(&self) -> bool {
        self.input.jump && !self.last_input.jump
    }
    fn light_just_pressed(&self) -> bool {
        self.input.light_attack && !self.last_input.light_attack
    }
    fn hevy_just_pressed(&self) -> bool {
        self.input.heavy_attack && !self.last_input.heavy_attack
    }
    #[allow(dead_code)]
    fn spec_just_pressed(&self) -> bool {
        self.input.special && !self.last_input.special
    }
    
    pub fn update(&mut self, char_sheet: &Character, map: &crate::game::MapInformation,delta: &f32) {
        const GRAVITY: f32 = 0.1;

        self.update_animation(char_sheet);

        let multiplayer =
            if 0.0 < self.velocity[1] {
                if self.input.jump {
                    0.5
                } else {
                    3.0
                }
            } else {
                if self.input.down {
                    5.0
                } else {
                    1.0
                }
            };

        self.velocity[1] -= delta * GRAVITY * multiplayer;
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
                        self.air_jump = char_sheet.air_jump_count;
                        self.air_action = Self::AIR_ACTION_DEFAULT;
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

        let new_location = Math::add_vec(&self.position,&self.velocity);
        if Math::distance(&new_location,&self.position) < 0.0001 {
            self.velocity = [0.0,0.0];
        } else {
            self.position = new_location;
        }
        match self.state.clone() {
            State::Actionable => {
                if self.hevy_just_pressed() {
                    if self.airborn {
                        if 0 < self.air_action {
                            self.air_action -= 1;
                            self.change_animation(AnimationState::AirBornHeavyAttack);
                        }
                    } else {
                        self.change_animation(AnimationState::HeavyAttack);
                    }
                } else if self.light_just_pressed() {
                    if self.airborn {
                        self.change_animation(AnimationState::AirBornLightAttack);
                    } else {
                        self.change_animation(AnimationState::LightAttack);
                    }
                } else {
                    // Input Logick
                    if self.jump_just_pressed() && (!self.airborn || 0 < self.air_jump)  {
                        self.velocity[1] = char_sheet.jump;
                        if self.airborn {
                            self.air_jump -= 1;
                        }  
                    }
                    let input_direction = self.input.dir.clone();
                    match &input_direction {
                        Some(some) => {
                            self.direction = some.clone();
                            let potentional_acel = char_sheet.aceleration * delta * some.to_float();
                            if (self.velocity[0] + potentional_acel).abs() < char_sheet.max_speed{
                                self.velocity[0] += potentional_acel;
                            }
                        },
                        None => {},
                    }
                    // Idle Animation Logick
                    if self.airborn {
                        if self.velocity[1] > 0.0 {
                            self.change_animation(AnimationState::Rizing);
                        } else {
                            self.change_animation(AnimationState::Falling);
                        }
                    } else {
                        match input_direction {
                            Option::Some(_) => {self.change_animation(AnimationState::Running)},
                            Option::None => {self.change_animation(AnimationState::Idling)},
                        }
                    }
                }
            },
            State::HitStun(wait, enemy) => {
                if wait <= 0 {
                    self.state = State::Actionable;
                } else {
                    self.state = State::HitStun(wait-1,enemy);
                }
            },
            State::Acting => {},
        }

        if Math::distance(&self.position,&[0.0,0.0]) > 5.0 {
            self.reset();
        }
        self.last_input = self.input.clone();
    }
    fn change_animation(&mut self, anim: AnimationState) {
        if self.animation != anim {
            self.animation = anim;
            self.animation_hold = 0;
            self.animation_frame = 0;
        }
    }
    fn apply_frame_event(&mut self,event: &FrameEvent, source: &Self, character: &Character) {
        match event {
            FrameEvent::MultiplyVelocity(vec) => {
                self.velocity[0] *= vec[0];
                self.velocity[1] *= vec[1];
            },
            FrameEvent::SetVelocityFromPoint(_vec,_force) => {
                todo!();
            },
            FrameEvent::AddVelocityFromPoint(_vec,_force) => {
                todo!();
            },
            FrameEvent::SetVelocity(vec) => {
                self.velocity[0] = vec[0] * source.direction.to_float();
                self.velocity[1] = vec[1];
            },
            FrameEvent::AddVelocity(vec) => {
                const MINIMAL: f32 = 0.1;
                if source.object_id == self.object_id {
                    self.velocity[0] += vec[0];
                    self.velocity[1] += vec[1];
                } else {
                    self.velocity[0] += vec[0] * source.direction.to_float() * (self.damage + MINIMAL)/character.weight;
                    self.velocity[1] += vec[1] * (self.damage + MINIMAL)/character.weight;
                }
            },
            FrameEvent::MoveBy(pos) => {
                self.position[0] += pos[0] * source.direction.to_float() * self.damage;
                self.position[1] += pos[1] * self.damage;
            },
            FrameEvent::DealDamage(amount) => {
                self.damage += amount;
            },
            FrameEvent::ApplyHitStun(amount) => {
                self.change_animation(AnimationState::Damadged);
                let enemy_id = (source.object_id,source.animation_frame);
                match self.state.clone() {
                    State::HitStun(old,_) => {
                        self.state = State::HitStun(old + amount,enemy_id);
                    },
                    _ => {
                        self.state = State::HitStun(*amount,enemy_id);
                    },
                }
            },
            FrameEvent::ChangeColisionState(col_state) => {
                self.vournable = col_state.clone();
            },
            FrameEvent::ChangeActionState(state) => {
                self.state = state.clone();
            },
        }
    }
    fn update_animation(&mut self,character: &Character) {
        let anim: &Vec<AnimationFrame> = &character.get_animations(&self.animation);

        if self.animation_hold >= anim[self.animation_frame].hold {
            if self.animation_frame + 1 >= anim.len() {
                self.animation_frame = 0;
                if !self.animation.looping() {
                    self.animation = AnimationState::Idling;
                }
            } else {
                self.animation_frame += 1;
            }
            self.animation_hold = 0;
        } else {
            self.animation_hold += 1
        }

        for event in &anim[self.animation_frame].events {
            self.apply_frame_event(event,&self.clone(),character);
        }
    }
    pub fn get_hitboxes(&self, char_sheet: &HashMap<u32,Character>) -> (Vec<HitSircle>,Vec<ColisionSircle>) {
        let character = char_sheet.get(&self.character).expect("Character that is trying to be rendered not found");
        let anim = character.get_animations(&self.animation);
        let dir = match self.direction {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        };

        let mut hit: Vec<HitSircle> = anim[self.animation_frame].hit_sircles.clone();
        hit.iter_mut().for_each(|x| {x.colision_shape.position[0] *= dir;});

        let mut hurt: Vec<ColisionSircle> = anim[self.animation_frame].hurt_sircles.clone();
        hurt.iter_mut().for_each(|x| {x.colision_shape.position[0] *= dir;});

        (hit,hurt)
    }
    pub fn draw_hurtbox(&self,display: &mut Display<WindowSurface>,frame_display: &mut glium::Frame,char_sheet: &HashMap<u32,Character>) {
        const BLUISH: [f32;4] = [0.1,0.0,1.0,1.0];
        for hurt_sir in &self.get_hitboxes(char_sheet).1 {
            hurt_sir.colision_shape.draw(display,frame_display,&self.position,BLUISH);
        }
    }
    pub fn draw_hitbox(&self,display: &mut Display<WindowSurface>,frame_display: &mut glium::Frame,char_sheet: &HashMap<u32,Character>) {
        const REDISH: [f32;4] = [1.0,0.0,0.1,1.0];
        for hit_sir in &self.get_hitboxes(char_sheet).0 {
            hit_sir.colision_shape.draw(display,frame_display,&self.position,REDISH);
        }
    }
    pub fn draw_colision_box(&self,display: &mut Display<WindowSurface>,frame_display: &mut glium::Frame,char_sheet: &HashMap<u32,Character>) {
        let character = char_sheet.get(&self.character).expect("Character that is trying to be rendered not found");
        const GREENISH: [f32;4] = [0.0,0.3,0.6,1.0];
        character.colider.draw(display,frame_display,&self.position,GREENISH)
    }
    pub fn draw(&self,display: &mut Display<WindowSurface>,frame_display: &mut glium::Frame,char_sheet: &HashMap<u32,Character>) {
        let character = char_sheet.get(&self.character).expect("Character that is trying to be rendered not found");
        let position = self.position;
        let frame = self.animation_frame;
        character.get_animations(&self.animation).get(frame).expect(&format!("This character doesnt have frame: {frame}, in animation: {}",self.animation.to_str()))
            .texture.draw_on(display, frame_display, position,&self.direction);
    }
}
