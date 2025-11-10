use crate::base::vector2::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Sircle {
    pub radius: f32,
    pub position: Vector2
}
