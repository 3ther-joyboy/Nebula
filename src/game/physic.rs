use serde::{Serialize, Deserialize};
use crate::{
    base::vector2::Vector2,
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
pub struct ColisionSircle {
    pub colision_shape: Sircle,
    pub state: ColisionState,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct HitSircle {
    colision_shape: Sircle,
    impact_events: Vec<FrameEvent>
}
#[derive(Serialize, Deserialize, Clone)]
pub enum FrameEvent{

    SetVelocity(Vector2),
    AddVelocity(Vector2),

    SetVelocityFromPoint(Vector2,f32),
    AddVelocityFromPoint(Vector2,f32),

    MoveTo(Vector2),
    DealDamage(f32),
    ApplyHitStun(f32),
    ChangeColisionState(ColisionState,u32),
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AnimationFrame {
    pub hurt_sircles: Vec<ColisionSircle>,
    pub hit_sircles: Vec<HitSircle>,
    pub events: Vec<FrameEvent>,
    pub texture: Texture,
    pub hold: u8,
}
#[derive(Serialize, Deserialize, Clone,PartialEq)]
pub enum AnimationState {
    Idling,
    Running,

    Jump,
    Rizing,
    Falling,

    LightAttack,
    HeavyAttack,
    
    AirBornLightAttack,
    AirBornHeavyAttack,
}
#[derive(Debug,Serialize, Deserialize, Clone)]
pub enum Direction {
    Left = -1,
    Right = 1,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum State {
    Actionable,
    Acting,
    HitStun(u32),
}
