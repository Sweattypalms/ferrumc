use ferrumc::handle_packet;
use crate::server::connection::{Connection};
use crate::err::FerrumcError;
use crate::server::connection::ConnectionState;
use crate::server::inbound::{handshake::handshake, ping::ping};
use crate::server::inbound::{login_start::login_start};
use log::trace;


pub struct PacketData<'a>{
    pub connection: &'a mut Connection, // Reference to the connection
    pub id: u8,
    pub bytes: Vec<u8>,
}

pub async fn handle_packet(connection: &mut Connection, id: u8, bytes: Vec<u8>) -> Result<(), FerrumcError> {
    let packet_data = PacketData {
        connection,
        id,
        bytes,
    };

    handle_packet!(packet_data,
        ConnectionState::Handshaking => 0x00 => handshake,
        ConnectionState::Status => 0x01 => ping,
        ConnectionState::Login => 0x00 => login_start
    )
}