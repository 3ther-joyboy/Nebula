use serde::{Serialize, Deserialize};
use crate::base::vector2::Vector2;

pub trait Renderable {
    fn draw(&self);
    fn draw_on(&self, position: Vector2);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Texture {
}
impl Renderable for Texture {
    fn draw(&self) {
        todo!()
    }
    fn draw_on(&self,position: Vector2) {
        todo!()
    }
}

 
