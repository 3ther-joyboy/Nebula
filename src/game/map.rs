use serde::{
    Serialize,
    Deserialize,
};
use uuid::Uuid;
use crate::{
    game::character::CharacterInstance,
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
            characters: Vec::new(),
            statics: Vec::new(),
        }
    }
}
