use std::{
    collections::HashMap,
};
use crate::game::CharacterInput;
use serde::{
    Serialize,
    Deserialize,
};
use crate::{
    base::vector2::*,
    base::sircle::*,
    game::physic::*,
    client::renderer::*,
};
use crate::base::*;

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
            hurt_sircles: vec![ColisionSircle {state: ColisionState::Vulnerable ,colision_shape: Sircle{radius:0.5,position: Vector2{x:0.0,y:0.5}}}],
            hit_sircles: Vec::new(),
            events: Vec::new(),
            texture: Texture{},
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
    fn new() -> Character {
        Character {
            id: 0,
            name: String::new(),
            weight: 1.0,
            air_jump_count: 1,
            aceleration: 10.0,
            max_speed: 5.0,
            colider: Sircle {radius: 0.3, position: Vector2{x:0.0,y:0.3}},
            animations: Animations::test(),
        }
    }
    pub fn load(char_id: u32) -> Character {
        if char_id == 0 {
            return Self::new();
        }
        todo!();
    }
    pub fn load_all() -> HashMap<u32,Character> {
        todo!();
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterInstance {
    pub character: Option<u32>,
    object_id: usize,

    position: Vector2,
    velocity: Vector2,
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
    pub fn new(character: Option<u32>) -> CharacterInstance {
        CharacterInstance {
            character,
            object_id: 0,

            position: Vector2::ZERO,
            velocity: Vector2::ZERO,
            direction: Direction::Left,
            airborn: true,
            vournable: ColisionState::Vulnerable,
            state: State::Actionable,
            damage: 0.0,
            animation: AnimationState::Idling(1),

            input: CharacterInput::new(),
        }
    }
    pub fn update(&mut self, _char_sheet: &Character) {
        match self.input.dir.clone() {
            Some(Direction::Left) => {self.position.x -= 1.0},
            Some(Direction::Right) => {self.position.x += 1.0},
            None => {},
        }
    }
    fn draw(&self,char_sheet: &HashMap<u32,Character>,offset: Vector2) {
        if let Some(char_sheet_id) = self.character {
            let anim_tree = &char_sheet.get(&char_sheet_id)
                .expect("Character by ID: {char_sheet_id} not found.\nTrying to render unknow character.")
                .animations;
            let position = offset + self.position.clone();
            match self.animation {
                AnimationState::Idling(frame) => anim_tree.idling[frame].texture.draw_on(position),
                AnimationState::Running(frame) => anim_tree.running[frame].texture.draw_on(position),
                AnimationState::Jump(frame) => anim_tree.jump[frame].texture.draw_on(position),
                AnimationState::Rizing(frame) => anim_tree.rizing[frame].texture.draw_on(position),
                AnimationState::Falling(frame) => anim_tree.falling[frame].texture.draw_on(position),
                AnimationState::LightAttack(frame) => anim_tree.light_attack[frame].texture.draw_on(position),
                AnimationState::HeavyAttack(frame) => anim_tree.heavy_attack[frame].texture.draw_on(position),
                AnimationState::AirBornLightAttack(frame) => anim_tree.air_born_light_attack[frame].texture.draw_on(position),
                AnimationState::AirBornHeavyAttack(frame) => anim_tree.air_born_heavy_attack[frame].texture.draw_on(position),
            }
        } else {
            panic!("You are trying to render instance of a player without character definition");
        }
    }
}
