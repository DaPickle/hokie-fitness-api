use std::env;

pub struct Config {
    pub ip_address: String,
}

impl Config {
    pub fn new() -> Self {
        let ip = env::var("IP").unwrap_or(String::from("127.0.0.1"));
        let port = env::var("PORT").unwrap_or(String::from("8080"));

        Self {
            ip_address: format!("{ip}:{port}")
        }
    }
}