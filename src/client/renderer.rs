use serde::{Serialize, Deserialize};
use winit::application::ApplicationHandler;
use std::collections::HashMap;
use crate::game::character::Character;


use image::ImageReader;
use image::GenericImageView;

use winit::{
    event::WindowEvent,
    event_loop::{
        ActiveEventLoop,
    },
    window::{
        Window,
        WindowId,
    },
};
use std::sync::mpsc::{Receiver,Sender};
// use std::collections::HashMap;
use glium::{
    glutin::surface::WindowSurface,
    Surface,
    Display,
};

use crate::game::map::Map;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32;2],
    tex_coords: [f32;2],
}
implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    pub fn new(position: [f32;2],tex_coords: [f32;2]) -> Vertex {
        Vertex {
            position,
            tex_coords,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Texture {
    path: String,
    matrix: [[f32;4];4],
    offset: [f32;2],
}
impl Texture {
    const OPEN_GL: &str = "./assets/opengl.png";

    pub fn new() -> Texture {
        Texture {
            path: String::from(Self::OPEN_GL),
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            offset: [0.0,0.0],
        }
    }
    pub fn draw_on(&self ,display: &mut Display<WindowSurface>,frame: &mut glium::Frame,post: [f32;2]) {

        let image = ImageReader::open(self.path.clone()).unwrap().decode().unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = glium::texture::Texture2d::new(display, image).unwrap();

        let shape = vec![
            Vertex::new([-0.5 + post[0], -0.5 + post[1]],[0.0, 0.0]),
            Vertex::new([ 0.5 + post[0], -0.5 + post[1]],[1.0, 0.0]),
            Vertex::new([ 0.5 + post[0],  0.5 + post[1]],[1.0, 1.0]),

            Vertex::new([ 0.5 + post[0],  0.5 + post[1]],[1.0, 1.0]),
            Vertex::new([-0.5 + post[0],  0.5 + post[1]],[0.0, 1.0]),
            Vertex::new([-0.5 + post[0], -0.5 + post[1]],[0.0, 0.0]),
        ];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            in vec2 tex_coords;
            out vec2 v_tex_coords;

            uniform mat4 matrix;

            void main() {
                v_tex_coords = tex_coords;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coords;
            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;
        let uniforms = uniform! {matrix: self.matrix, tex: &texture};

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let program_err = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None);

        frame.draw(&vertex_buffer, &indices, &program_err.unwrap(), &uniforms, &Default::default()).unwrap();


    }
}

#[allow(dead_code)]
pub struct GameRanderer {
    input_channel: Sender<WindowEvent>,
    map_channel: Receiver<Map>,
    window: Window,
    display: Display<WindowSurface>,

    character_sheet: HashMap<u32,Character>,
}
impl GameRanderer {
    pub fn new(map_channel: Receiver<Map>, input_channel: Sender<WindowEvent>, window: Window, display: Display<WindowSurface> ) -> GameRanderer {
        GameRanderer {
            map_channel,
            input_channel,
            window,
            display,
            character_sheet: Character::load_all(),
        }
    }
}
impl ApplicationHandler for GameRanderer {
    fn resumed(&mut self, _: &ActiveEventLoop) {
        let mut target = self.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.finish().unwrap();
    }
    fn window_event(&mut self, window: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        self.input_channel.send(event.clone()).expect("Sending inputs to network core failed");
        match event {
            WindowEvent::CloseRequested => window.exit(),
            WindowEvent::RedrawRequested => {
                let mut map_it = self.map_channel.try_iter().peekable();
                while let Some(map) = map_it.next() && map_it.peek().is_none() {
                    let mut target = self.display.draw();
                    target.clear_color(1.0, 1.0, 1.0, 1.0);

                    for (_,character) in map.characters {
                        character.draw(&mut self.display,&mut target,&self.character_sheet);
                    }


                    target.finish().unwrap();
                }
            },
            _ => {},
        }

    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window.request_redraw();
    }
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct VertexTest {
    position: [f32; 2],
}
implement_vertex!(VertexTest, position);
impl VertexTest {
    #[allow(dead_code)]
    pub fn draw_simple_triangle(display: &mut Display<WindowSurface>,frame: &mut glium::Frame,post: (f32,f32)) {
        let shape = vec![
            VertexTest { position: [ post.0/100.0 - 0.1,  post.1/100.0 - 0.1] },
            VertexTest { position: [ post.0/100.0,  post.1/100.0 + 0.1] },
            VertexTest { position: [ post.0/100.0 + 0.1,  post.1/100.0 - 0.1] },
        ];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,&Default::default()).unwrap();
    }
}

