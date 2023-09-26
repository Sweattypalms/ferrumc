use crate::{err::FerrumcError, varint_to_int, Connection, ConnectionState};

use log::{error, trace};

pub async fn handle_handshake(
    packet: Vec<u8>,
    connection: &mut Connection,
) -> Result<Option<Vec<u8>>, FerrumcError> {
    let packet_length = packet[0];
    let protocol_version = varint_to_int(&packet[2..4]);
    let server_address = String::from_utf8(packet[4..(packet_length - 2) as usize].to_vec())
        .expect("Unable to parse server address");
    let server_port = u16::from_be_bytes([
        packet[packet_length as usize - 2],
        packet[packet_length as usize - 1],
    ]);
    let next_state = packet[packet_length as usize];
    trace!("Protocol Version: {}", protocol_version);
    trace!("Server Address: {}", server_address);
    trace!("Server Port: {}", server_port);
    trace!("Next State: {}", next_state);
    connection.state = match next_state {
        1 => ConnectionState::Status,
        2 => ConnectionState::Login,
        _ => {
            error!("Invalid next state: {}", next_state);
            return Err(FerrumcError::InvalidPacketID);
        }
    };
    return Ok(None);
}