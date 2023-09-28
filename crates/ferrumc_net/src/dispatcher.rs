use std::io::Cursor;
use crate::err::FerrumcError;
use log::{error, info, trace};

use crate::{Connection, CursorExt};
use crate::ConnectionState;
use crate::Handlers::handshake;
use crate::Handlers::login::login;
use crate::Handlers::status::status;

pub(crate) async fn dispatch(
    mut connection: &mut Connection,
    mut packet: Vec<u8>,
) -> Result<Option<Vec<u8>>, FerrumcError> {
    trace!("Current state: {:?}", connection.state);
    let mut cursor = Cursor::new(&packet);
    let packet_length = cursor.read_varint()?;
    let packet_id = cursor.read_varint()?;
    match connection.state {
        ConnectionState::Handshaking => match packet_id {
            0x00 => {
                trace!("Received handshake packet");
                handshake::handle_handshake(packet, &mut connection).await
            }

            _ => {
                error!("Unknown packet ID: {} for handshake stage", packet_id);
                return Err(FerrumcError::InvalidPacketID);
            }
        },
        ConnectionState::Status => match packet_id{
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
                error!("Unknown packet ID: {} for status stage", packet_id);
                return Err(FerrumcError::InvalidPacketID);
            }
        },
        ConnectionState::Login => match packet_id {
            0x00 => {
                trace!("Received login start packet");
                login(packet, &mut connection).await
            }
            _ => {
                error!("Unknown packet ID: {} for login stage", packet_id);
                return Err(FerrumcError::InvalidPacketID);
            }
        },
        ConnectionState::Play => {
            todo!();
        }
    }
}