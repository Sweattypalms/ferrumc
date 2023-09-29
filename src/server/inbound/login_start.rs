use std::io::Cursor;
use colored::Colorize;
use log::{info, trace};
use crate::err::FerrumcError;
use crate::server::outbound::login_success::login_success;
use crate::server::packet::PacketData;
use crate::utils::MinecraftReaderExt;

pub async fn login_start(packet_data: PacketData<'_>) -> Result<(), FerrumcError> {
    trace!("Login start packet received");

    let mut cursor = Cursor::new(packet_data.bytes);
    let username = cursor.read_varstring()?;

    trace!("Username: {}", username);

    // "Accepted connection from ({}:{})", addr.ip().to_string().green(), addr.port().to_string().blue()
    let address = packet_data.connection.stream.peer_addr()?;
    let host = address.ip().to_string();
    let port = address.port();
    info!("{}[{}:{}] logged in", username, host.green(), port.to_string().blue());

    login_success(packet_data.connection, username).await?;

    Ok(())
}