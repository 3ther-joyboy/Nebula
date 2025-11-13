use chrono::{Utc,DateTime,Datelike};
use std::fmt::Display;
use std::io::BufReader;
use std::net::TcpStream;
use crate::game::physic::Direction;
use serde::{
    Serialize,
    Deserialize,
};

pub fn get_responce<T: for<'a> Deserialize<'a>>(lines: &mut std::io::Lines<BufReader<&TcpStream>>) -> Option<T> {
    while let Some(Ok(x)) = lines.next() && !x.trim().is_empty() {
    }
    let mut obj_str = String::new();
    while let Some(Ok(x)) = lines.next() && !x.trim().is_empty(){
        obj_str.push_str(&x);

        if let Ok(output) = serde_json::from_str::<T>(format!("{obj_str}}}").as_str()) {
            return Some(output);
        }
    }
    None
}

pub enum ResponseStatus {
    Ok,
    Error(String),
    None,
    Forbiden,
}
impl ResponseStatus {
    pub fn to_string(&self) -> String {
        match self {
            ResponseStatus::Ok => String::from("HTTP/1.1 200 OK"),
            ResponseStatus::Error(msg) => format!("HTTP/1.1 500 {msg}"),
            ResponseStatus::None => String::from("HTTP/1.1 404 NOT FOUND"),
            ResponseStatus::Forbiden => String::from("HTTP/1.1 403 FORBIDEN ERROR"),
        }
    }
}
pub enum BodyType {
    HTML,
    JSON,
}
impl BodyType {
    pub fn to_string(&self) -> String {
        match self {
            BodyType::HTML => String::from("text/html"),
            BodyType::JSON => String::from("application/json"),
        }
    }
}

pub struct Response {
    status: ResponseStatus,
    host_name: String, 
    date: DateTime<Utc>,
    body_type: BodyType,
    body: String,
}
impl Response {
    pub fn new(status: ResponseStatus, body_type: BodyType, body: &str) -> Response {
        Response {
            status,
            host_name: String::from("Tree house"),
            date: Utc::now(),
            body_type,
            body: body.to_string(),
        }
    }
    pub fn to_string(&self) -> String {
        let status = self.status.to_string();
        let server_name = self.host_name.clone();
        let content_leanght = self.body.len();
        let body_type = self.body_type.to_string();
        let contents = self.body.clone();

        let (_, year) = self.date.year_ce();
        let curr_time = format!(
            "{}-{:02}-{:02} {}",
            year,
            self.date.month(),
            self.date.day(),
            self.date.time(),
        );


        format!(
"{status}
Server: {server_name}
Date: {curr_time}
Content-Length: {content_leanght}
Content-Type: {body_type}

{contents}"
        )
    }
}
#[derive(Debug,Default,Serialize, Deserialize, Clone)]
pub struct CharacterInput {
    pub dir: Option<Direction>,
    pub light_attack: bool,
    pub heavy_attack: bool,
    pub special: bool,
    pub jump: bool,
}
impl CharacterInput {
    pub fn new() -> CharacterInput {
        CharacterInput {
            dir: Option::None,
            light_attack: false,
            heavy_attack: false,
            special: false,
            jump: false,
        }
    } 
}
#[derive(Debug,Default,Serialize, Deserialize, Clone)]
pub struct GameControlPacket {
    pub input: CharacterInput,
    pub server_password: String,
    pub player: String,
}
impl GameControlPacket {
    pub fn new() -> GameControlPacket {
        GameControlPacket {
            input: CharacterInput::new(),
            server_password: String::from("lorem ipsum"),
            player: String::from("franta"),
        }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_string(input: String) -> Option<GameControlPacket> {
        if let Ok(output) = serde_json::from_str::<Self>(input.as_str()) {
            Some(output)
        } else {
            None
        }
    }
} 
#[derive(Debug,Default,Serialize, Deserialize, Clone)]
pub struct JoinRequest {
    pub server_password: String,
    pub player_name: String,
}
impl JoinRequest {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_string(input: String) -> Option<Self> {
        if let Ok(output) = serde_json::from_str::<Self>(input.as_str()) {
            Some(output)
        } else {
            None
        }
    }
}
