use crate::base::*;
use crate::renderer::*;

enum ColisionState {
    Vulnerable,
    Invincible,
    UnTouchable,
}
struct ColisionSircle {
    colision_shape: Sircle,
    state: ColisionState,
}
struct HitSircle {
    colision_shape: Sircle,
    impact_events: Vec<FrameEvent>
}
enum FrameEvent{
    SetVelocity(Vector2),
    AddVelocity(Vector2),
    MoveTo(Vector2),

    DealDamage(f32),
    ApplyHitStun(f32),
    ChangeColisionState(ColisionState,u32),
}
struct AnimationFrame {
    hurt_sircles: Vec<ColisionSircle>,
    hit_sircles: Vec<HitSircle>,
    events: Vec<FrameEvent>,
    texture: Texture,
}
enum AnimationState {
    Idling(usize),
    Running(usize),

    Jump(usize),
    Rizing(usize),
    Falling(usize),

    LightAttack(usize),
    // LightAttackNeutral(usize),
    HeavyAttack(usize),
    // HeavyAttackNeutral(usize),
    
    AirBornLightAttack(usize),
    // AirBornightAttackNeutral(usize),
    AirBornHeavyAttack(usize),
    // AirBornHeavyAttackNeutral(usize),
}
struct Animations {
    idling: Vec<AnimationFrame>,
    running: Vec<AnimationFrame>,
    jump: Vec<AnimationFrame>,
    rizing: Vec<AnimationFrame>,
    falling: Vec<AnimationFrame>,
    light_attack: Vec<AnimationFrame>,
    // light_attack_neutral: Vec<AnimationFrame>,
    heavy_attack: Vec<AnimationFrame>,
    // heavy_attack_neutral: Vec<AnimationFrame>,
    air_born_light_attack: Vec<AnimationFrame>,
    // air_bornight_attack_neutral: Vec<AnimationFrame>,
    air_born_heavy_attack: Vec<AnimationFrame>,
    // air_born_heavy_attack_neutral: Vec<AnimationFrame>,
}
impl Animations {
    pub fn new() -> Animations {
        Animations {
            idling: Vec::new(),
            running: Vec::new(),
            jump: Vec::new(),
            rizing: Vec::new(),
            falling: Vec::new(),
            light_attack: Vec::new(),
            // light_attack_neutral: Vec::new(),
            heavy_attack: Vec::new(),
            // heavy_attack_neutral: Vec::new(),
            air_born_light_attack: Vec::new(),
            // air_bornight_attack_neutral: Vec::new(),
            air_born_heavy_attack: Vec::new(),
            // air_born_heavy_attack_neutral: Vec::new(),
        }
    }
}
struct Character {
    id: usize,
    name: String,
    weight: f32,
    air_jump_count: u32,
    aceleration: f32,
    max_speed: f32,
    colider: Sircle,

    animations: Animations,
}
enum Direction {
    Left,
    Right,
}
enum State {
    Actionable,
    Acting,
    HitStun(u32),
}
impl Character {
    pub fn new() -> Character {
        Character {
            id: 0,
            name: String::new(),
            weight: 1.0,
            air_jump_count: 1,
            aceleration: 10.0,
            max_speed: 5.0,
            colider: Sircle {radius: 0.3, position: Vector2{x:0.0,y:0.3}},
            animations: Animations::new(),
        }
    }
}
pub struct CharacterInstance<'a> {
    character: Option<&'a Character>,
    object_id: usize,

    position: Vector2,
    velocity: Vector2,
    direction: Direction,
    airborn: bool,

    vournable: ColisionState,
    state: State,
    damage: f32,

    animation: AnimationState,
}
impl CharacterInstance<'_> {
    pub fn new() -> CharacterInstance<'static> {
        CharacterInstance {
            character: None,

            object_id: 0,
            position: Vector2::ZERO,
            velocity: Vector2::ZERO,
            direction: Direction::Left,
            airborn: true,
            vournable: ColisionState::Vulnerable,
            state: State::Actionable,
            damage: 0.0,
            animation: AnimationState::Idling(1),
        }
    }
}
impl Renderable for CharacterInstance<'_> {
    fn draw(&self) {
        if let Some(char_sheet) = self.character {
            let anim_tree = &char_sheet.animations;
            match self.animation {
                AnimationState::Idling(frame) => anim_tree.idling[frame].texture.draw(),
                AnimationState::Running(frame) => anim_tree.running[frame].texture.draw(),
                AnimationState::Jump(frame) => anim_tree.jump[frame].texture.draw(),
                AnimationState::Rizing(frame) => anim_tree.rizing[frame].texture.draw(),
                AnimationState::Falling(frame) => anim_tree.falling[frame].texture.draw(),
                AnimationState::LightAttack(frame) => anim_tree.light_attack[frame].texture.draw(),
                // LightAttackNeutral(frame) => anim_tree.light_attack_neutral[frame].texture.draw(),
                AnimationState::HeavyAttack(frame) => anim_tree.heavy_attack[frame].texture.draw(),
                // HeavyAttackNeutral(frame) => anim_tree.XXX[frame].texture.draw(),
                AnimationState::AirBornLightAttack(frame) => anim_tree.air_born_light_attack[frame].texture.draw(),
                // AirBornightAttackNeutral(frame) => anim_tree.XXX[frame].texture.draw(),
                AnimationState::AirBornHeavyAttack(frame) => anim_tree.air_born_heavy_attack[frame].texture.draw(),
                // AirBornHeavyAttackNeutral(frame) => anim_tree.XXX[frame].texture.draw(),
            }
        } else {
            panic!("You are trying to render instance of a player without character definition");
        }
    }
}
