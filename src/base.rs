use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(dead_code)]
pub struct Sircle {
    pub position: Vector2,
    pub radius: f32,
}

#[allow(dead_code)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
#[allow(dead_code)]
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
