use serde::{Serialize, Deserialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl Add for Vector2 {
    type Output = Vector2;
    fn add(self,second: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + second.x,
            y: self.y + second.y,
        }
    }
}
impl Vector2 {
    pub fn new(x: f32,y: f32) -> Vector2 {
        Vector2 {x,y}
    }
    pub const ZERO: Vector2 = Vector2{x: 0.0, y: 0.0};
    pub const LEFT: Vector2 = Vector2{x: -1.0, y: 0.0};
    pub const RIGHT: Vector2 = Vector2{x: 1.0, y: 0.0};
    pub const UP: Vector2 = Vector2{x: 0.0, y: 1.0};
    pub const DOWN: Vector2 = Vector2{x: 0.0, y: -1.0};
    pub const RU: Vector2 = Vector2{x: 1.0, y: 1.0};
    pub const LU: Vector2 = Vector2{x: -1.0, y: 1.0};
    pub const RD: Vector2 = Vector2{x: 1.0, y: -1.0};
    pub const LD: Vector2 = Vector2{x: -1.0, y: -1.0};

}
