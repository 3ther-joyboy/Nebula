use serde::{
    Serialize,
    Deserialize,
};
use crate::game::{
    character::CharacterInstance,
    Character,

    physic::Direction,
    physic::ColisionPlane,
};
use crate::client::renderer::Texture;
use glium::{
    glutin::surface::WindowSurface,
    Display,
};
use std::{
    collections::HashMap,
    fs::{
        self,
        File,
    },
    io::Read,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct MapInformation {
    background: Option<Texture>,
    stage: Option<Texture>,
    foreground: Option<Texture>,
    pub render_colission_boxes: bool,
    pub statics: Vec<ColisionPlane>
}
impl MapInformation {
    pub fn draw_background(&self,display: &mut Display<WindowSurface>,frame_display: &mut glium::Frame) {
        if let Some(tex) = &self.background {
            tex.draw(display,frame_display);
        }
    }
    pub fn draw_foreground(&self,display: &mut Display<WindowSurface>,frame_display: &mut glium::Frame) {
        if let Some(tex) = &self.stage {
            tex.draw_on(display,frame_display,[0.0,0.0],&Direction::Right);
        }
        if let Some(tex) = &self.foreground {
            tex.draw(display,frame_display);
        }
    }
    pub fn default() -> MapInformation {
        MapInformation {
            background: Option::None,
            stage: Option::None,
            foreground: Option::None,
            render_colission_boxes: true,
            statics: Vec::new(),
        }
    }
    fn load_textures(&mut self, display: &mut Display<WindowSurface>) {
        if let Some(tex) = &mut self.background {
            tex.load_texture(display);
        } 
        if let Some(tex) = &mut self.stage {
            tex.load_texture(display);
        } 
        if let Some(tex) = &mut self.foreground {
            tex.load_texture(display);
        } 
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    const MAP_PATH: &str = "./assets/maps/";
    pub fn load(map_id: usize, display_option: &mut Option<&mut Display<WindowSurface>>) -> Option<MapInformation> {
        if map_id == 0 {
            let mut default = Self::default();
            if let Some(ref mut display) = display_option.as_mut() {
                default.load_textures(display);
            }
            return Some(default);
        }
        let mut character_json = String::new();
        if let Ok(mut file) = File::open(format!("{0}{map_id}.json",Self::MAP_PATH)) && let Ok(_) = file.read_to_string(&mut character_json){
            let char_result = serde_json::from_str::<Self>(&character_json);
            match char_result {
                Ok(mut output) => {
                    if let Some(ref mut display) = display_option.as_mut() {
                        output.load_textures(display);
                    }
                    return Some(output);
                },
                Err(error) => {
                    println!("{error:?}");
                    return Option::None;
                }
            }
        }
        Option::None
    }
    pub fn load_all(display: Option<&mut Display<WindowSurface>>) -> HashMap<usize,MapInformation> {
        let mut display: Option<&mut Display<WindowSurface>> = display;

        let mut out = HashMap::new();
        out.insert(0,Self::load(0,&mut display).expect("Loading a default map failed.."));
        if let Ok(items_directory) = fs::read_dir(Self::MAP_PATH) {
            for character_files in items_directory {
                if  let Ok(something) = character_files &&
                    let Ok(file_type) = something.file_type() && // has a file type
                    file_type.is_file() && // is a file (not dir or linked)
                    let Ok(name) = something.file_name().into_string() && // is possible to convert
                                                                          // name in to regurall ascii
                    name.len() > ".json".len() && // has more characters then .json thingie
                    name[name.len()-5..] == *".json" && // is last few chars ".json"
                    let Ok(id_number) = name[..name.len()-5].parse::<usize>() && // parse the the
                                                                               // name in to a number
                    let Some(map_info) = Self::load(id_number,&mut display) { // is possible to load the character
                    out.insert(id_number,map_info);
                } 
            }
        }
        out
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    pub counter: usize,
    #[serde(skip_serializing,skip_deserializing)]
    pub current_id: u32,
    pub characters: HashMap<u32,CharacterInstance>,
    pub map_id: usize,
}
impl Map {
    pub fn new_istance(&mut self, character: u32) -> u32 {
        self.characters.insert(self.current_id,CharacterInstance::new(character,self.current_id));
        let out = self.current_id;
        self.current_id += 1;
        out
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn new(map_id: usize) -> Map {
        Map {
            counter: 0,
            characters: HashMap::new(),
            current_id: 0,
            map_id,
        }
    }
    pub fn test() -> Map {
        Map {
            counter: 0,
            characters: HashMap::new(),
            current_id: 0,
            map_id: 0,
        }
    }
    pub fn set_inputs(&mut self,players: HashMap<String,crate::game::Player>) {
        for (_,player) in players {
            if let Some(id) = player.instance && let Some(instance) = &mut self.characters.get_mut(&id) {
                instance.input = player.input;
            }
        }
    }
    pub fn update(&mut self, char_sheet: &HashMap<u32,Character>,map_pool: &HashMap<usize,MapInformation>) {
        if let Some(map) = map_pool.get(&self.map_id) {
            for (_,player) in &mut self.characters.iter_mut() {
                if let Some(sheet) = &char_sheet.get(&player.character) {
                    player.update(sheet,map);
                }
            }
        }
    }
}
