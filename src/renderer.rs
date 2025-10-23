pub trait Renderable {
    fn draw(&self);
}

pub struct Texture {
}
impl Renderable for Texture {
    fn draw(&self) {
        todo!()
    }

}
