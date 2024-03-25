use std::env;

pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        let port = env::var("PORT").unwrap_or(String::from("8080")).parse::<u16>().unwrap_or(8080);

        Self {
            port
        }
    }
}