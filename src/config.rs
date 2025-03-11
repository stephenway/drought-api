use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub api_port: u16,
}

pub fn load() -> Config {
    dotenv().ok();
    Config {
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        api_port: env::var("API_PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap(),
    }
}

