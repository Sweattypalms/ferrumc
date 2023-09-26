use crate::err::FerrumcError;
use log::{error, info, trace};

use crate::Connection;
use crate::ConnectionState;
use crate::Handlers::handshake;
use crate::Handlers::login::login;
use crate::Handlers::status::status;

pub(crate) async fn dispatch(
    mut connection: &mut Connection,
    mut packet: Vec<u8>,
) -> Result<Option<Vec<u8>>, FerrumcError> {
    trace!("Current state: {:?}", connection.state);
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
        ConnectionState::Login => match packet[1] {
            0x00 => {
                trace!("Received login start packet");
                login(packet, &mut connection).await
            }
            _ => {
                error!("Unknown packet ID: {} for login stage", packet[0]);
                return Err(FerrumcError::InvalidPacketID);
            }
        },
        ConnectionState::Play => {
            todo!();
        }
    }
}