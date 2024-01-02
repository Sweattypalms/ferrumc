use std::io::Write;
use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

use crate::error::Error::ConfigError as CfgError;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

impl ServerConfig {
    pub fn new() -> Result<Self> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config"))
            .build();

        let settings = match settings {
            Ok(settings) => settings,
            Err(err) => {
                if check_if_error_is_not_found(&err) {
                    println!("Config file not found, creating...");
                    create_config_file()?;
                    return Self::new();
                }
                return Err(CfgError(err));
            }
        };

        let de_settings: ServerConfig = match settings.try_deserialize() {
            Ok(settings) => settings,
            Err(e) => {
                println!("There was an error deserializing the config.");

                if let Some(field) = check_if_error_is_missing_field(&e) {
                    println!("Missing field {} in config file", field);
                    println!("Would you like to create a new config file? (y/n)");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    return if input.trim() == "y" {
                        println!("Creating new config file...");
                        create_config_file()?;
                        Self::new()
                    } else {
                        println!("Aborting...");
                        Err(CfgError(e))
                    }
                }

                return Err(CfgError(e));
            }
        };

        Ok(de_settings)
    }
}


fn check_if_error_is_not_found(err: &ConfigError) -> bool {
    if let ConfigError::Foreign(err) = err {
        if err.is::<std::io::Error>() {
            return true;
        }
    }
    false
}
fn check_if_error_is_missing_field(err: &ConfigError) -> Option<String> {
    if let ConfigError::Message(err) = err {
        if err.contains("missing field") {
            let field = err.replace("missing field ", "");
            return Some(field);
        }
    }
    None
}
fn create_config_file() -> Result<()> {
    let path = std::path::Path::new("config.toml");
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    let mut file = std::fs::File::create(path)?;
    let contents = toml::to_string(&ServerConfig::default())?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}


impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 25565, // Minecraft default port
        }
    }
}
