use uuid::Uuid;
use ferrumc::create_packet;
use crate::err::FerrumcError;
use crate::server::player_connection::{Connection, ConnectionState};
use crate::utils::MinecraftWriterExt;
use tokio::io::AsyncWriteExt;
use crate::server::outbound::login_play::login_play;

pub async fn login_success(connection: &mut Connection, username: String) -> Result<(), FerrumcError> {
    let namespace_uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, "OfflinePlayer".as_bytes());

    let uuid = Uuid::new_v3(&namespace_uuid, username.as_bytes());

    let mut buffer = Vec::new();

    buffer.extend_from_slice(uuid.as_bytes());

    buffer.write_varstring(&username)?;

    let raw = create_packet!(0x02, buffer)?;

    connection.stream.flush().await?;
    connection.write(&raw).await?;

    login_play(connection).await?;

    connection.state = ConnectionState::Play;

    Ok(())
}