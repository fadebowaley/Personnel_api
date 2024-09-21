// src/config.rs

use std::env;

pub struct Config {
    pub db_url: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        Config {
            db_url: env::var("DB_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080),
        }
    }
}

