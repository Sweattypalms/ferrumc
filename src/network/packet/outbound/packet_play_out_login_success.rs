use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::network::packet::OutboundPacket;
use crate::utils::write_varint;

pub struct PacketPlayOutLoginSuccess {
    pub username: String,
}

#[async_trait]
impl OutboundPacket for PacketPlayOutLoginSuccess {
    async fn serialize(&self) -> Result<Vec<u8>> {
        // Generate a namespace UUID for "OfflinePlayer" using the DNS namespace as a base
        let namespace_uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, "OfflinePlayer".as_bytes());

        // Generate a v3 UUID using the player's username and the "OfflinePlayer" namespace
        let uuid = Uuid::new_v3(&namespace_uuid, self.username.as_bytes());

        let username = self.username.clone();

        let mut temp_buffer = Vec::new();

        // Write Packet ID (0x02)
        write_varint(&mut temp_buffer, 0x02).await;

        // Write UUID
        temp_buffer.extend_from_slice(uuid.as_bytes());

        // Write username
        write_varint(&mut temp_buffer, username.len() as i32).await;
        temp_buffer.extend_from_slice(username.as_bytes());

        let mut final_buffer = Vec::new();

        // Write the total packet length as VarInt
        write_varint(&mut final_buffer, temp_buffer.len() as i32).await;

        final_buffer.extend_from_slice(&temp_buffer);

        // println!("final_buffer: {:?}", final_buffer);

        Ok(final_buffer)
    }

    fn get_id(&self) -> u32 {
        0x02
    }
}