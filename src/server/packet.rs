use ferrumc::handle_packet;
use crate::server::player_connection::{Connection};
use crate::err::FerrumcError;
use crate::server::player_connection::ConnectionState;
use crate::server::inbound::{handshake::handshake, ping::ping};
use crate::server::inbound::{login_start::login_start, player_position::player_position};
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
        ConnectionState::Handshaking => 0 => handshake,
        ConnectionState::Status => 1 => ping,
        ConnectionState::Login => 0 => login_start,
        ConnectionState::Play => 17 => player_position
    )
}