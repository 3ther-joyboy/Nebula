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

pub struct Player {
    id: Uuid,
    last_ping: usize,
    name: String,
    instance: Option<usize>,   
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            id: Uuid::new_v4(),
            last_ping: 0,
            name,
            instance: None,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    pub counter: usize,
    characters: Vec<CharacterInstance>,
    statics: Vec<()>,
}
impl Map {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn test() -> Map {
        Map {
            counter: 0,
            characters: vec![CharacterInstance::new(Some(0))],
            statics: Vec::new(),
        }
    }
    pub fn update(&mut self, char_sheet: &HashMap<u32,Character>) {
        for player in &mut self.characters {
            let some = &char_sheet.get(&player.character.unwrap()).unwrap(); // todo!
            player.update(some);
        }
    }
}
