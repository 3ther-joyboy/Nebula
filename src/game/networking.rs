use chrono::{Utc,DateTime,Datelike};

pub enum ResponseStatus {
    Ok,
    Error(String),
    None,
}
impl ResponseStatus {
    pub fn to_string(&self) -> String {
        match self {
            ResponseStatus::Ok => String::from("HTTP/1.1 200 OK"),
            ResponseStatus::Error(msg) => format!("HTTP/1.1 500 {msg}"),
            ResponseStatus::None => String::from("HTTP/1.1 404 NOT FOUND"),
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
