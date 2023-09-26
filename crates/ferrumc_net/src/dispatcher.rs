use crate::err::FerrumcError;
use log::{error, trace};

use crate::Connection;
use crate::ConnectionState;
use crate::Handlers::handshake;
use crate::Handlers::status::status;

pub(crate) async fn dispatch(
    mut connection: &mut Connection,
    mut packet: Vec<u8>,
) -> Result<Option<Vec<u8>>, FerrumcError> {
    let mut outbound_packet: Vec<u8> = Vec::new();
    match connection.state {
        ConnectionState::Handshaking => match packet[1] {
            0x00 => {
                trace!("Received handshake packet");
                handshake::handle_handshake(packet, &mut connection).await
            }

            _ => {
                error!("Unknown packet ID: {} for handshake stage", packet[0]);
                return Err(FerrumcError::InvalidPacketID);
            }
        },
        ConnectionState::Status => match packet[1] {
            0x00 => {
                trace!("Received status request packet");
                status(&mut connection).await
            }
            0x01 => {
                trace!("Received status ping packet");
                // Just return the same packet lol
                Ok(Some(packet))
            }
            _ => {
                error!("Unknown packet ID: {} for status stage", packet[0]);
                return Err(FerrumcError::InvalidPacketID);
            }
        },
        ConnectionState::Login => {
            todo!();
        }
        ConnectionState::Play => {
            todo!();
        }
    }
}