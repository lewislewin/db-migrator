use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub r#type: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub databases: Databases,
}

#[derive(Deserialize)]
pub struct Databases {
    pub source: DatabaseConfig,
    pub target: DatabaseConfig,
}

impl AppConfig {
    pub fn from_file(file_path: &str) -> Self {
        let config_content = fs::read_to_string(file_path)
            .expect("Failed to read configuration file");
        toml::from_str(&config_content)
            .expect("Failed to parse configuration file")
    }
}
