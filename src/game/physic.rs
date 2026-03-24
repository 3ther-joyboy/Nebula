use serde::{Serialize, Deserialize};
use crate::{
    client::renderer::*,
    base::sircle::*,
};

#[derive(Serialize, Deserialize, Clone)]
pub enum ColisionState {
    Vulnerable,
    Invincible,
    UnTouchable,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ColisionPlane {
    pub position: [f32;2],
    pub size: f32,
    pub rotation: Orientation,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ColisionSircle {
    pub colision_shape: Sircle,
    pub state: ColisionState,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct HitSircle {
    pub colision_shape: Sircle,
    pub impact_events: Vec<FrameEvent>
}
type Vector2 = [f32;2];
#[derive(Serialize, Deserialize, Clone)]
pub enum FrameEvent{

    MultiplyVelocity(Vector2),
    SetVelocity(Vector2),
    AddVelocity(Vector2),

    SetVelocityFromPoint(Vector2,f32),
    AddVelocityFromPoint(Vector2,f32),

    MoveBy(Vector2),
    DealDamage(f32),
    ApplyHitStun(u32),

    ChangeColisionState(ColisionState),
    ChangeActionState(State),
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AnimationFrame {
    #[serde(default)]
    pub hurt_sircles: Vec<ColisionSircle>,
    #[serde(default)]
    pub hit_sircles: Vec<HitSircle>,
    #[serde(default)]
    pub events: Vec<FrameEvent>,
    pub texture: Texture,
    pub hold: u8,
}
#[derive(Serialize, Deserialize, Clone,PartialEq)]
pub enum AnimationState {
    Damadged,

    Idling,
    Running,

    Rizing,
    Falling,

    LightAttack,
    HeavyAttack,
    
    AirBornLightAttack,
    AirBornHeavyAttack,
}
impl AnimationState {
    pub fn to_str(&self) -> &str {
        match self {
            AnimationState::Damadged => "Hurt",
            AnimationState::Idling => "Idling",
            AnimationState::Running => "Running",
            AnimationState::Rizing => "Rizing",
            AnimationState::Falling => "Falling",
            AnimationState::LightAttack => "Light Attack",
            AnimationState::HeavyAttack => "Heavy Attack",
            AnimationState::AirBornLightAttack => "Air Light Attack",
            AnimationState::AirBornHeavyAttack => "Air Heavy Attack",
        } 
    }
    pub fn looping(&self) -> bool {
        match self {
            AnimationState::Damadged => {true},
            AnimationState::Idling => {true},
            AnimationState::Running => {true},
            AnimationState::Rizing => {true},
            AnimationState::Falling => {true},
            AnimationState::LightAttack => {false},
            AnimationState::HeavyAttack => {false},
            AnimationState::AirBornLightAttack => {false},
            AnimationState::AirBornHeavyAttack => {false},
        } 
    }
}
#[derive(Debug,Serialize, Deserialize, Clone)]
pub enum Direction {
    Left,
    Right,
}
impl Direction {
    pub fn to_float(&self) -> f32 {
        match self {
            Direction::Left => {-1.0},
            Direction::Right => {1.0},
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum State {
    Actionable,
    Acting,
    // frames_left, (user, frame_hit)
    HitStun(u32,(u32,usize)),
}
