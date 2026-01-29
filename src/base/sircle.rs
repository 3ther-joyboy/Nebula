use serde::{Serialize, Deserialize};
use glium::{
    glutin::surface::WindowSurface,
    Display,
};
use crate::client::renderer::GameRanderer;
use crate::base::Math;

#[derive(Serialize, Deserialize, Clone)]
pub struct Sircle {
    pub radius: f32,
    pub position: [f32;2],
}
impl Sircle {
    pub fn draw(&self,dis: &mut Display<WindowSurface>,frm: &mut glium::Frame,
        position: &[f32;2],
        color: [f32;4],
        ) {
        let pos = Math::add_vec(position,&self.position);
        GameRanderer::draw_triangle_on(dis,frm,
            (
                Math::add_vec(&pos,&[0.0,self.radius]),
                Math::add_vec(&pos,&[0.0,-self.radius]),
                Math::add_vec(&pos,&[self.radius,0.0]),
                ),
            color);
        GameRanderer::draw_triangle_on(dis,frm,
            (
                Math::add_vec(&pos,&[0.0,self.radius]),
                Math::add_vec(&pos,&[0.0,-self.radius]),
                Math::add_vec(&pos,&[-self.radius,0.0]),
                ),
            color);
    }
}
