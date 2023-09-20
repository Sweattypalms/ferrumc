use anyhow::Result;

use tokio::net::TcpStream;



use crate::player::Player;


pub mod state;

pub async fn handle_connection(stream: TcpStream) -> Result<()> {
    println!("New connection from {}", stream.peer_addr()?);

    let player = Player::new(stream);
    player.init().await?;

    Ok(())
}