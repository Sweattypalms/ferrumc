use crate::err::FerrumcError;
use lazy_static::lazy_static;
use log::{error, info, trace};
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub motd: String,
    pub max_players: i32,
}

impl ServerConfig {
    pub fn default() -> ServerConfig {
        ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 25565,
            motd: "A FerrumC server".to_string(),
            max_players: 100,
        }
    }
}

pub fn read_config(filename: &str) -> Result<ServerConfig, FerrumcError> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: ServerConfig = toml::from_str(&contents)?;

    Ok(config)
}

pub fn write_config(filename: &str, config: &ServerConfig) -> Result<(), FerrumcError> {
    let contents = toml::to_string(config)?;
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn get_config() -> ServerConfig {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
        trace!("Failed to get CARGO_MANIFEST_DIR, using current directory instead.");
        ".".to_string()
    });

    let config_file = format!("{}/{}", root, "config.toml");

    let config = match read_config(&config_file) {
        Ok(config) => config,
        Err(FerrumcError::TomlDeserialize(err)) => {
            error!("Failed to parse config file: {}", err);
            std::process::exit(1);
        }
        Err(err) => {
            error!("Failed to read config file: {}", err);
            info!("Creating default config file...");
            let config = ServerConfig::default();
            write_config(&config_file, &config).unwrap_or_else(|err| {
                error!("Failed to write config file: {}", err);
                std::process::exit(1);
            });
            config
        }
    };
    config
}

lazy_static! {
    pub static ref CONFIG: ServerConfig = get_config();
}
