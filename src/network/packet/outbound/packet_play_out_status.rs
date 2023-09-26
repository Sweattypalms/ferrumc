use async_trait::async_trait;
use serde::Serialize;

use crate::network::packet::OutboundPacket;
use crate::utils::write_varint;

use anyhow::Result;
use log::trace;

pub struct PacketPlayOutStatus {
    pub motd: String,
}

#[async_trait]
impl OutboundPacket for PacketPlayOutStatus {
    async fn serialize(&self) -> Result<Vec<u8>> {
        let mut sample = Vec::new();
        let sample_player = Sample {
            name: "Recore_".to_string(),
            id: "2b3414ed-468a-45c2-b113-6c5f47430edc".to_string(),
        };
        sample.push(sample_player);

        let payload = JsonResponse {
            version: Version {
                name: "FerrumC - 1.17.1".to_string(),
                protocol: 756,
            },
            players: Players {
                max: 100,
                online: sample.len() as i32,
                sample,
            },
            description: Description {
                text: self.motd.clone(),
            },
        };

        trace!("Status payload: {:?}", payload);

        let json_bytes = serde_json::to_vec(&payload)?;

        let mut temp_buffer = vec![];

        // Write Packet ID (0x00)
        write_varint(&mut temp_buffer, 0x00).await;

        // Write the length of the JSON string as VarInt
        write_varint(&mut temp_buffer, json_bytes.len() as i32).await;

        // Write JSON string bytes
        temp_buffer.extend_from_slice(&*json_bytes);

        let packet_length = temp_buffer.len() as i32;

        let mut final_buffer = vec![];

        // Write the total packet length as VarInt
        write_varint(&mut final_buffer, packet_length).await;

        final_buffer.extend_from_slice((&temp_buffer).as_ref());

        trace!("final play_out_status buffer: {:?}", final_buffer);

        Ok(final_buffer)
    }

    fn get_id(&self) -> u32 {
        todo!()
    }
}
#[derive(Serialize, Debug)]
pub struct JsonResponse {
    version: Version,
    players: Players,
    description: Description,
}

#[derive(Serialize, Debug)]
pub struct Version {
    name: String,
    protocol: i32,
}

#[derive(Serialize, Debug)]
pub struct Players {
    max: i32,
    online: i32,
    sample: Vec<Sample>,
}

#[derive(Serialize, Debug)]
pub struct Description {
    text: String,
}

#[derive(Serialize, Debug)]
pub struct Sample {
    name: String,
    id: String,
}