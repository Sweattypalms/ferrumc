use anyhow::Result;
use log::info;

use tokio::net::TcpStream;

use crate::player::Player;

pub mod state;

pub async fn handle_connection(stream: TcpStream) -> Result<()> {
    info!("New connection from {}", stream.peer_addr()?);

    // let player = Player::new(stream);
    // player.init().await?;
    ferrumc_net::handle_connection(stream).await;

    Ok(())
}