use serde::{Serialize, Deserialize};
use winit::application::ApplicationHandler;
use std::collections::HashMap;
use crate::game::physic::Direction;

use crate::game::character::Character;
use crate::game::map::MapInformation;

use image::ImageReader;
use crate::game::physic::Orientation;

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

#[derive(Serialize, Deserialize)]
pub struct Texture {
    path: String,
    offset: [f32;2],

    scale: f32,
    position: [u32;2],
    dimensions: [u32;2],
    
    #[serde(skip_serializing,skip_deserializing)]
    texture: Option<glium::texture::Texture2d>,
}
impl Clone for Texture {
    fn clone(&self) -> Texture {
        Texture {
            path: self.path.clone(),
            offset: self.offset.clone(),
            texture: Option::None,
            scale: self.scale.clone(),

            position: self.position.clone(),
            dimensions: self.dimensions.clone(),
        }
    }
}
impl Texture {
    const OPEN_GL: &str = "./assets/opengl.png";
    const FLOAT_TO_PIXELS: f32 = 250.0; // how manny pixels should be 1 unit
    pub fn new() -> Texture {
        Texture {
            path: String::from(Self::OPEN_GL),
            offset: [0.0,0.0],
            position: [0,0],
            scale: 1.0,
            dimensions: [600,300],
            texture: Option::None,
        }
    }
    fn new_vertex_shape(&self,dim: (f32,f32), off: (f32,f32),dir: &Direction) -> Vec<Vertex> {
        let dir = match dir {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        };
        let (offset_x, offset_y) = ((off.0 as f32 + self.offset[0] * dir * self.scale) * dim.0, (off.1 as f32 + self.offset[1] * self.scale) * dim.1);
        let (pos_x, pos_y) = (self.dimensions[0] as f32 * dim.0 * self.scale * dir, self.dimensions[1] as f32 * self.scale * dim.1);
        vec![
            Vertex::new([offset_x,offset_y],[0.0, 0.0]),
            Vertex::new([offset_x + pos_x, offset_y],[1.0, 0.0]),
            Vertex::new([offset_x + pos_x, offset_y + pos_y],[1.0, 1.0]),

            Vertex::new([offset_x + pos_x, offset_y + pos_y],[1.0, 1.0]),
            Vertex::new([offset_x, offset_y + pos_y],[0.0, 1.0]),
            Vertex::new([offset_x,offset_y],[0.0, 0.0]),
        ]
    }
    pub fn load_texture(&mut self,  display: &mut Display<WindowSurface>) {
        let image = ImageReader::open(self.path.clone()).unwrap().decode().unwrap().crop(self.position[0],self.position[1],self.dimensions[0],self.dimensions[1]).to_rgba8(); //todo!();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = glium::texture::Texture2d::new(display, image).unwrap();
        self.texture = Some(texture);
    }
    const VERTEX_SHADER_SRC: &str = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;

        out vec2 v_tex_coords;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    const FRAGMENT_SHADER_SRC: &str = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            vec4 texColor = texture(tex, v_tex_coords);
            if(texColor.a < 1.0)
                discard;
            color = vec4(texColor.r,texColor.g,texColor.b,1.0);
        }
    "#;
    pub fn draw(&self ,display: &mut Display<WindowSurface>,frame: &mut glium::Frame) {
        let shape = vec![
            Vertex::new([-1.0, -1.0],[0.0, 0.0]),
            Vertex::new([1.0, -1.0],[1.0, 0.0]),
            Vertex::new([1.0, 1.0],[1.0, 1.0]),

            Vertex::new([1.0, 1.0],[1.0, 1.0]),
            Vertex::new([-1.0, 1.0],[0.0, 1.0]),
            Vertex::new([-1.0, -1.0],[0.0, 0.0]),
        ];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let uniforms = uniform! {
            tex: self.texture.as_ref().expect("No texture loaded"),
        };
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let program_err = glium::Program::from_source(display, Self::VERTEX_SHADER_SRC, Self::FRAGMENT_SHADER_SRC, None);
        frame.draw(&vertex_buffer, &indices, &program_err.unwrap(), &uniforms, &Default::default()).unwrap();
    }
    pub fn draw_on(&self ,display: &mut Display<WindowSurface>,frame: &mut glium::Frame,post: [f32;2],dir: &Direction) {
        let (x,y) = display.get_framebuffer_dimensions();
        let scaled_dimensions = (1.0/(x as f32 *2.0),1.0/(y as f32 *2.0));

        let shape = self.new_vertex_shape(scaled_dimensions,(post[0] * Self::FLOAT_TO_PIXELS,post[1] * Self::FLOAT_TO_PIXELS),dir);
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let uniforms = uniform! {
            tex: self.texture.as_ref().expect("No texture loaded"),
        };

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let program_err = glium::Program::from_source(display, Self::VERTEX_SHADER_SRC, Self::FRAGMENT_SHADER_SRC, None);

        frame.draw(&vertex_buffer, &indices, &program_err.unwrap(), &uniforms, &Default::default()).unwrap();
    }
}

pub struct GameRanderer {
    input_channel: Sender<WindowEvent>,
    map_channel: Receiver<Map>,
    window: Window,
    display: Display<WindowSurface>,

    character_sheet: HashMap<u32,Character>,
    map_pool: HashMap<usize,MapInformation>,
}
impl GameRanderer {
    pub fn new(map_channel: Receiver<Map>, input_channel: Sender<WindowEvent>, window: Window, mut display: Display<WindowSurface> ) -> GameRanderer {
        GameRanderer {
            map_channel,
            input_channel,
            window,
            character_sheet: Character::load_all(Some(&mut display)),
            map_pool: MapInformation::load_all(Some(&mut display)),
            display,
        }
    }
}
impl GameRanderer {
    pub fn draw_triangle_on( display: &mut Display<WindowSurface>, frame: &mut glium::Frame, post: ([f32;2],[f32;2],[f32;2])) {
        #[derive(Copy, Clone)]
        struct Ver {
            position: [f32;2],
        }
        implement_vertex!(Ver, position);
        let (x,y) = display.get_framebuffer_dimensions();
        let (x,y) = (x as f32, y as f32);
        let shape = vec![
            Ver {position: [post.0[0]*Texture::FLOAT_TO_PIXELS/x,post.0[1]*Texture::FLOAT_TO_PIXELS/y]},
            Ver {position: [post.1[0]*Texture::FLOAT_TO_PIXELS/x,post.1[1]*Texture::FLOAT_TO_PIXELS/y]},
            Ver {position: [post.2[0]*Texture::FLOAT_TO_PIXELS/x,post.2[1]*Texture::FLOAT_TO_PIXELS/y]},
        ];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;
        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
        let program_err = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None);

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        frame.draw(&vertex_buffer, &indices, &program_err.unwrap(), &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
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

                    let map_info = self.map_pool.get(&map.map_id);
                    if let Some(map) = map_info {
                        map.draw_background(&mut self.display,&mut target)
                    }
                    for (_,character) in map.characters {
                        character.draw(&mut self.display,&mut target,&self.character_sheet);
                    }
                    if let Some(map) = map_info {
                        map.draw_foreground(&mut self.display,&mut target);
                        if map.render_colission_boxes {
                            for collision in &map.statics {
                                let pos = collision.position;
                                let off = collision.size/2.0;
                                let (x,y) = match &collision.rotation {
                                    Orientation::Left => (0.0,1.0),
                                    Orientation::Down => (1.0,0.0),
                                    Orientation::Right => (0.0,-1.0),
                                    Orientation::Up => (-1.0,0.0),
                                };
                                Self::draw_triangle_on(&mut self.display,&mut target,
                                        (
                                            [pos[0] + off*x,pos[1] + off*y],
                                            [pos[0] - off*x,pos[1] - off*y],
                                            [pos[0] + 0.1*y,pos[1] + 0.1*x],
                                        )
                                    );
                            }
                        }
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
