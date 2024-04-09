use std::env;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        // load environment variables from .env file
        dotenv().ok();

        let port = env::var("PORT").unwrap_or(String::from("8080")).parse::<u16>().unwrap_or(8080);

        let name = "PORT";
        match env::var(name) {
            Ok(v) => println!("{}: {}", name, v),
            Err(e) => panic!("${} is not set ({})", name, e)
        }

        Self {
            port
        }
    }
}