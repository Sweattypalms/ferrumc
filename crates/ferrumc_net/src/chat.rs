use crate::packet::PacketData;
use ferrumc_utils::err::FerrumcError;
use ferrumc_utils::utils::MinecraftReaderExt;
use log::trace;
use std::io::Cursor;

pub async fn chat_message(packet_data: &mut PacketData<'_>) -> Result<(), FerrumcError> {
    let mut cursor = Cursor::new(&packet_data.bytes);

    let message = cursor.read_varstring()?;
    trace!("Chat message: {}", message);

    Ok(())
}