use crate::create_packet;
use crate::login_play::login_play;
use crate::packet::PacketData;
use crate::player_connection::ConnectionState;
use colored::Colorize;
use ferrumc_utils::err::FerrumcError;
use ferrumc_utils::utils::{MinecraftReaderExt, MinecraftWriterExt};
use log::{info, trace};
use std::io::Cursor;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn login_start(packet_data: PacketData<'_>) -> Result<(), FerrumcError> {
    trace!("Login start packet received");

    let mut cursor = Cursor::new(packet_data.bytes);
    let username = cursor.read_varstring()?;

    trace!("Username: {}", username);

    // "Accepted connection from ({}:{})", addr.ip().to_string().green(), addr.port().to_string().blue()
    let address = packet_data.connection.stream.peer_addr()?;
    let host = address.ip().to_string();
    let port = address.port();
    info!(
        "{}[{}:{}] logged in",
        username,
        host.green(),
        port.to_string().blue()
    );

    let namespace_uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, "OfflinePlayer".as_bytes());

    let uuid = Uuid::new_v3(&namespace_uuid, username.as_bytes());

    let mut buffer = Vec::new();

    buffer.extend_from_slice(uuid.as_bytes());

    buffer.write_varstring(&username)?;

    let raw = create_packet!(0x02, buffer)?;

    packet_data.connection.stream.flush().await?;
    packet_data.connection.write(&raw).await?;

    login_play(packet_data.connection).await?;

    packet_data.connection.state = ConnectionState::Play;
    packet_data.connection.username = Some(username);
    packet_data.connection.uuid = Some(uuid);

    Ok(())
}