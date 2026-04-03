use serde::{Serialize, Deserialize};
use crate::{
    client::renderer::*,
    base::sircle::*,
};

/// Basic states that determinated if character can be hit or how it will resolve.
#[derive(Serialize, Deserialize, Clone)]
pub enum ColisionState {
    Vulnerable,
    Invincible,
    UnTouchable,
}
/// What orientation does a ColisionPlane have
#[derive(Serialize, Deserialize, Clone)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}
/// Static plane that can be colided with by the players.
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
/// Is being used for describing position and velocity
type Vector2 = [f32;2];
/// This is event that can happend while any animation is playing or can be applayed by a hitbox to
/// oponent.
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
/// Enum for switching between what animation is currently active.
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
    /// If the animation should start looping on its end.
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
/// What direction is the player looking.
#[derive(Debug,Serialize, Deserialize, Clone)]
pub enum Direction {
    Left,
    Right,
}
impl Direction {
    /// Enums cant hold negative inteegers, or float at all.
    pub fn to_float(&self) -> f32 {
        match self {
            Direction::Left => {-1.0},
            Direction::Right => {1.0},
        }
    }
}

/// State by wich teh player can act
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum State {
    /// Character can move, attack, do any action.
    Actionable,
    /// Character is in middle of action and has to wait untill the animations end.
    Acting,
    /// Counts how long will hit stun last and keeps track of waht user was the last to hit them to
    /// not allow multiple hits in one frame
    ///
    /// frames_left, (user, frame_hit)
    ///
    /// user -> Id, frame_hit -> What frame was that attack on
    HitStun(u32,(u32,usize)),
}
