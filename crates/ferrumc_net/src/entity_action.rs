use crate::packet::PacketData;
use ferrumc_utils::err::FerrumcError;
use ferrumc_utils::utils::MinecraftReaderExt;
use log::trace;
use std::io::Cursor;

pub async fn entity_action(packet_data: &mut PacketData<'_>) -> Result<(), FerrumcError> {
    let mut cursor = Cursor::new(&packet_data.bytes);

    let entity_id = cursor.read_varint()?;
    let action_id = cursor.read_varint()?;
    let jump_boost = cursor.read_varint()?;

    trace!(
        "Entity ID: {} | Action: {}",
        entity_id,
        match action_id {
            0 => "Start Sneaking",
            1 => "Stop Sneaking",
            2 => "Leave Bed",
            3 => "Start Sprinting",
            4 => "Stop Sprinting",
            5 => "Start Jumping With Horse",
            6 => "Stop Jumping With Horse",
            7 => "Open Horse Inventory",
            8 => "Start Flying With Elytra",
            _ => "Unknown",
        }
    );

    Ok(())
}