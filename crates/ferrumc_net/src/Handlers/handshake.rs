use std::io::{Cursor, Read};
use crate::{err::FerrumcError, varint_to_int, Connection, ConnectionState, read_varint, CursorExt};
use byteorder::{BigEndian, ReadBytesExt};
use tokio::io::AsyncWriteExt;
use log::{error, trace};

pub async fn handle_handshake(
    packet: Vec<u8>,
    connection: &mut Connection,
) -> Result<Option<Vec<u8>>, FerrumcError> {

    let mut cursor = Cursor::new(packet);

    let packet_length = cursor.read_varint()?;

    let packet_id = cursor.read_varint()?;

    let protocol_version = cursor.read_varint()?;

    let server_address = cursor.read_varstring()?;

    let server_port = cursor.read_u16_be()?;

    let next_state = read_varint(&mut cursor)?;

    trace!("Protocol Version: {}", protocol_version);
    trace!("Server Address: {}", server_address);
    trace!("Server Port: {}", server_port);
    trace!("Next State: {}", next_state);
    connection.state = match next_state {
        1 => ConnectionState::Status,
        2 => ConnectionState::Login,
        _ => {
            error!("Invalid next state: {}", next_state);
            return Err(FerrumcError::InvalidState);
        }
    };
    return Ok(None);
}