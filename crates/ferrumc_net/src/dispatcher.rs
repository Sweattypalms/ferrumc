use crate::err::FerrumcError;
use log::{error, trace};

use crate::Connection;
use crate::ConnectionState;

pub(crate) async fn dispatch(
    mut connection: &mut Connection,
    mut packet: Vec<u8>,
) -> Result<Option<Vec<u8>>, FerrumcError> {
    let mut outbound_packet: Vec<u8> = Vec::new();
    match connection.state {
        ConnectionState::Handshaking => match packet[0] {
            0x00 => {
                let protocol_version = packet[1];
                let server_address =
                    String::from_utf8(packet[2..packet.len() - 2].to_vec()).unwrap();
                let server_port =
                    u16::from_be_bytes([packet[packet.len() - 2], packet[packet.len() - 1]]);
                let next_state = packet[packet.len() - 3];
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

            _ => {
                error!("Unknown packet ID: {} for handshake stage", packet[0]);
                return Err(FerrumcError::InvalidPacketID);
            }
        },
        ConnectionState::Status => {
            todo!();
        }
        ConnectionState::Login => {
            todo!();
        }
        ConnectionState::Play => {
            todo!();
        }
    }
}