use crate::Connection;
use crate::ConnectionState;
use std::error::Error;

pub(crate) async fn dispatch(
    mut connection: Connection,
    mut packet: Vec<u8>,
) -> Result<Vec<u8>, Error> {
    let mut outbound_packet: Vec<u8> = Vec::new();
    match connection.state {
        ConnectionState::Handshaking => {
            todo!();
        }
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