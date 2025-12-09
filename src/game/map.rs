use serde::{
    Serialize,
    Deserialize,
};
use std::collections::HashMap;
use crate::{
    game::character::CharacterInstance,
    game::Character,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    pub counter: usize,
    pub current_id: u32,
    pub characters: HashMap<u32,CharacterInstance>,
    pub statics: Vec<()>,
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
    pub fn test() -> Map {
        Map {
            counter: 0,
            characters: HashMap::new(),
            current_id: 0,
            statics: Vec::new(),
        }
    }
    pub fn set_inputs(&mut self,players: HashMap<String,crate::game::Player>) {
        for (_,player) in players {
            if let Some(id) = player.instance && let Some(instance) = &mut self.characters.get_mut(&id) {
                instance.input = player.input;
            }
        }
    }
    pub fn update(&mut self, char_sheet: &HashMap<u32,Character>) {
        for (_,player) in &mut self.characters.iter_mut() {
            if let Some(sheet) = &char_sheet.get(&player.character) {
                player.update(sheet);
            }
        }
    }
}
