use crate::network::packet::OutboundPacket;
use crate::utils::{write_varint, write_varlong};
use async_trait::async_trait;

pub struct PacketPlayOutPong {
    pub payload: i64,
}

#[async_trait]
impl OutboundPacket for PacketPlayOutPong {
    async fn serialize(&self) -> Result<Vec<u8>, anyhow::Error> {
        let mut temp_buffer = Vec::new();

        write_varint(&mut temp_buffer, 0x09).await; // packet id + length of LONG

        write_varint(&mut temp_buffer, 0x01).await;

        write_varlong(&mut temp_buffer, self.payload).await?;

        let _packet_length = temp_buffer.len() as i32;

        let mut final_buffer = Vec::new();

        final_buffer.extend_from_slice(&temp_buffer);

        Ok(final_buffer)
    }

    fn get_id(&self) -> u32 {
        self.payload as u32
    }
}