use chrono::{Utc,DateTime,Datelike};
use std::net::TcpStream;
use crate::game::physic::Direction;
use serde::{
    Serialize,
    Deserialize,
};
use std::io::Read;

#[derive(Debug)]
pub struct Headers {
    pub request_type: String,
    pub path: String,
    pub http_type: Option<String>,

    pub content_type: Option<String>,
    pub user_agent: Option<String>,
    pub body_length: usize,
}
impl Headers {
    pub fn none() -> Headers {
        Headers {
            request_type: String::from("GET"),
            path: String::from("/"),
            http_type: None,
            content_type: None,
            user_agent: None,
            body_length: 0,
        }
    }
    pub fn new(stream: &mut TcpStream) -> Headers {
        let mut out = Self::none();
        if let Some(line) = get_line(stream) {
            let rq_line: Vec<String> = line.split_whitespace().map(|x|String::from(x)).collect();
            if let Some(word) = rq_line.get(0) {
                out.request_type = word.to_string();
            } 
            if let Some(word) = rq_line.get(1) {
                out.path = word.to_string();
            }
            out.http_type = rq_line.get(2).cloned();
        }
        while let Some(line) = get_line(stream) && line.len() > 1 {
            let rq_line: Vec<String> = line.split_whitespace().map(|x|String::from(x)).collect();


            if let Some(word) = rq_line.get(0) {
                match word.as_str() {
                    "Content-Type:" | "content-type:" => {out.content_type = rq_line.get(1).cloned();}
                    "Content-Length:" | "content-length:"=> {
                        if let Some(word) = rq_line.get(1) && let Ok(number) = word.parse::<usize>(){
                            out.body_length = number+1;
                        }
                    }
                    _ => {},
                }
            }

        }
        out
    }
}
pub fn get_line(stream: &mut TcpStream) -> Option<String> {
    let mut line = String::new();
    let mut buffer = [0;1];
    while let Ok(_) = stream.read_exact(&mut buffer) && let Ok(character) = str::from_utf8(&buffer) {
        if buffer[0] == b'\n' {return Some(line);}
        line.push_str(character)
    }
    None
}
pub fn get_responce<T: for<'a> Deserialize<'a>>(stream: &mut TcpStream, headers: Headers) -> Option<T> {
    if headers.body_length > 0 {
        let mut buffer = vec![0;headers.body_length-1];
        if let Ok(_) = stream.read_exact(&mut buffer) && let Ok(obj_str) = str::from_utf8(&buffer) {
            if let Ok(output) = serde_json::from_str::<T>(obj_str) {
                return Some(output);
            }
        }
    }
    None
}

pub enum ResponseStatus {
    Ok,
    Error,
    Unauthorized,
    ParseError,
    None,
    Forbiden,
    NotImplemented,
}
impl ResponseStatus {
    pub fn to_string(&self) -> String {
        format!("HTTP/1.1 {}", match self {
            ResponseStatus::Ok => "200 OK",
            ResponseStatus::ParseError => "400 PARSE ERROR",
            ResponseStatus::Unauthorized => "401 UNAUTHORIZED",
            ResponseStatus::Forbiden => "403 FORBIDEN ERROR",
            ResponseStatus::None => "404 NOT FOUND",
            ResponseStatus::Error => "500 SERVER ERROR",
            ResponseStatus::NotImplemented => "501 NOT IMPLEMENTED",
        })
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
    pub fn status(err: ResponseStatus) -> Response {
        Self::new(err,BodyType::JSON,"")
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
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn new() -> CharacterInput {
        CharacterInput {
            dir: Option::None,
            light_attack: false,
            heavy_attack: false,
            special: false,
            jump: false,
        }
    } 
    pub fn reset(&mut self) {
        self.light_attack = false;
        self.heavy_attack = false;
        self.special = false;
        self.jump = false;
    }
}
#[derive(Debug,Default,Serialize, Deserialize, Clone)]
pub struct GameControlPacket {
    pub input: CharacterInput,
    pub server_password: String,
    pub player: String,
}
impl GameControlPacket {
    pub fn new(server_password: String, player: String,input: CharacterInput) -> GameControlPacket {
        GameControlPacket {
            input,
            server_password,
            player,
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
pub struct CharacterSwitchRequest {
    pub server_password: String,
    pub player_name: String,
    pub character: Option<u32>,
}
impl CharacterSwitchRequest {
    pub fn new(server_password: String, player_name: String, character: Option<u32>) -> CharacterSwitchRequest {
        CharacterSwitchRequest {
            server_password,
            player_name,
            character,
        }
    }
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
#[derive(Debug,Default,Serialize, Deserialize, Clone)]
pub struct JoinRequest {
    pub server_password: String,
    pub player_name: String,
}
impl JoinRequest {
    pub fn new(server_password: String, player_name: String) -> JoinRequest {
        JoinRequest {
            server_password,
            player_name,
        }
    }
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
