use crate::packet::PacketData;
use crate::player_connection::ClientSettings;
use ferrumc_utils::err::FerrumcError;
use ferrumc_utils::utils::MinecraftReaderExt;
use log::trace;
use std::io::Cursor;

pub async fn client_settings(packet_data: &mut PacketData<'_>) -> Result<(), FerrumcError> {
    let mut cursor = Cursor::new(&packet_data.bytes);

    let locale = cursor.read_varstring()?;
    let view_distance = cursor.read_varint()?;
    let chat_mode = cursor.read_varint()?;
    let chat_colors = cursor.read_varint()?;
    let displayed_skin_parts = cursor.read_varint()?;
    let main_hand = cursor.read_varint()?;
    let text_filtering_enabled = cursor.read_varint()?;
    let data = ClientSettings {
        locale,
        view_distance,
        chat_mode,
        chat_colors: chat_colors != 0,
        displayed_skin_parts,
        main_hand,
        disable_text_filtering: text_filtering_enabled != 0,
    };
    packet_data.connection.client_settings = Some(data);
    trace!("Client settings retrieved");

    Ok(())
}