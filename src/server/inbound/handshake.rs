use std::io::Cursor;
use log::trace;
use crate::err::FerrumcError;
use crate::server::connection::ConnectionState;
use crate::server::outbound::status::status;
use crate::server::packet::PacketData;
use crate::utils::MinecraftReaderExt;

pub async fn handshake(packet_data: PacketData<'_>) -> Result<(), FerrumcError> {
    let mut cursor = Cursor::new(packet_data.bytes);
    let protocol_version = cursor.read_varint()?;
    let server_address = cursor.read_varstring()?;
    let server_port = cursor.read_u16_be()?;
    let next_state = cursor.read_varint()?;

    trace!("Protocol version: {}", protocol_version);
    trace!("Server address: {}", server_address);
    trace!("Server port: {}", server_port);
    trace!("Next state: {}", next_state);


    packet_data.connection.state = match next_state {
        1 => {
            status(packet_data.connection).await?;
            ConnectionState::Status
        }
        2 => ConnectionState::Login,
        _ => {
            trace!("Invalid next state: {}", next_state);
            return Err(FerrumcError::InvalidState);
        }
    };

    Ok(())
}