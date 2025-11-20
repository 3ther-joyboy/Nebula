use serde::{
    Serialize,
    Deserialize,
};
use std::collections::HashMap;
use uuid::Uuid;
use crate::{
    game::character::CharacterInstance,
    game::Character,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    pub counter: usize,
    characters: HashMap<u32,CharacterInstance>,
    statics: Vec<()>,
}
impl Map {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn test() -> Map {
        let mut characters = HashMap::new();
        characters.insert(0,CharacterInstance::new(Some(0)));
        Map {
            counter: 0,
            characters,
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
        for (_,player) in &mut self.characters {
            if let Some(id) = player.character && let Some(sheet) = &char_sheet.get(&id) {
                player.update(sheet);
            }
        }
    }
}
