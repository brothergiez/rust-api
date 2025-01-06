use dotenv::dotenv;
use std::env;

pub struct Config {
    pub server_host: String,
    pub server_port: String,
    pub log_level: String,
    pub timezone: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Config {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string()),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            timezone: env::var("APP_TIMEZONE").unwrap_or_else(|_| "UTC".to_string()),
        }
    }
}