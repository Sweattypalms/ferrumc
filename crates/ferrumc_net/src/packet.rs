use crate::chunk_data::set_pos_on_join;
use crate::handle_packet;
use crate::player_connection::Connection;
use crate::player_connection::ConnectionState;
use crate::{
    chat::chat_message, entity_action::entity_action, join_settings::client_settings,
    login_start::login_start, player_position::*,
};
use crate::{handshake::handshake, ping::ping};
use ferrumc_utils::err::FerrumcError;
use log::trace;

pub struct PacketData<'a> {
    pub connection: &'a mut Connection,
    // Reference to the connection
    pub id: u8,
    pub bytes: Vec<u8>,
}

pub async fn handle_packet(
    connection: &mut Connection,
    id: u8,
    bytes: Vec<u8>,
) -> Result<(), FerrumcError> {
    let mut packet_data = PacketData {
        connection,
        id,
        bytes,
    };

    handle_packet!(packet_data,
        ConnectionState::Handshaking => { 0 => handshake },
        ConnectionState::Status => { 1 => ping },
        ConnectionState::Login => { 0 => login_start },
        ConnectionState::Play => {
            3 => chat_message,
            5 => client_settings,
            17 => player_position,
            18 => player_position_and_rotation,
            19 => player_rotation,
            10 => set_pos_on_join,
            25 => set_on_ground,
            27 => entity_action
        }
    )
}