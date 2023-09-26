use anyhow::Error;
use async_trait::async_trait;
use log::{error, trace};
use std::io::Cursor;
use tokio::io::AsyncWriteExt;

use crate::network::packet::outbound::packet_play_out_pong::PacketPlayOutPong;
use crate::network::packet::{InboundPacket, OutboundPacket};
use crate::player::Connection;
use crate::utils::{read_varint, read_varlong};

pub struct PacketPlayInPing {
    pub payload: i64,
}

#[async_trait]
impl InboundPacket for PacketPlayInPing {
    async fn deserialize(bytes: Vec<u8>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut cursor = Cursor::new(bytes);

        let _packet_length = read_varint(&mut cursor)?;

        // Read Packet ID (should be 1 for a Ping packet)
        let packet_id = read_varint(&mut cursor)?;
        trace!("packet_id: {:?} | packet length: {:?}", packet_id, _packet_length);

        if packet_id != 1 {
            return Err(Error::msg("Invalid packet ID"));
        }

        // Read payload
        let payload = read_varlong(&mut cursor)?;
        trace!("data: {:?}", payload);

        Ok(Self { payload })
    }

    fn get_id(&self) -> u32 {
        0x01
    }

    async fn handle(&self, connection: &mut Connection) {
        trace!("Received Ping packet with payload: {}", self.payload);
        let pong_packet = PacketPlayOutPong {
            payload: self.payload,
        };
        if let Ok(e) = pong_packet.serialize().await {
            //println!("Sending pong packet: {:?}", e);
            if let Err(e) = connection.stream.write_all((&e).as_ref()).await {
                error!("There was an error sending the pong packet: {:?}", e);
            }
        } else {
            error!("There was an error serializing the pong packet");
        }

        if let Err(_) = connection.stream.flush().await {
            error!("There was an error flushing the stream");
        }
        if let Err(_) = connection.stream.shutdown().await {
            error!("There was an error shutting down the stream");
        }
    }
}