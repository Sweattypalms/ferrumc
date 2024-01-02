#![allow(unused)]

use crate::prelude::*;

mod error;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Initializing server...");

    let config = crate::utils::config::ServerConfig::new()?;

    println!("Config: {:?}", config);

    Ok(())
}