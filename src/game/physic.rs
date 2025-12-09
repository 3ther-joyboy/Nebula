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
type HoldSize = u8;
#[derive(Serialize, Deserialize, Clone)]
pub struct AnimationFrame {
    pub hurt_sircles: Vec<ColisionSircle>,
    pub hit_sircles: Vec<HitSircle>,
    pub events: Vec<FrameEvent>,
    pub texture: Texture,
    pub hold: HoldSize,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum AnimationState {
    Idling(usize,HoldSize),
    Running(usize,HoldSize),

    Jump(usize,HoldSize),
    Rizing(usize,HoldSize),
    Falling(usize,HoldSize),

    LightAttack(usize,HoldSize),
    HeavyAttack(usize,HoldSize),
    
    AirBornLightAttack(usize,HoldSize),
    AirBornHeavyAttack(usize,HoldSize),
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
