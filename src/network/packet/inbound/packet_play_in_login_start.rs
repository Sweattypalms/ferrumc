use async_trait::async_trait;

use crate::network::packet::{InboundPacket, OutboundPacket};
use crate::network::packet::outbound::packet_play_out_login_success::PacketPlayOutLoginSuccess;
use crate::player::Connection;

use crate::utils::truncate_packet_header;

pub struct PacketPlayInLoginStart {
    pub username: String,
}


#[async_trait]
impl InboundPacket for PacketPlayInLoginStart {
    async fn deserialize(bytes: Vec<u8>) -> Result<Self, anyhow::Error> where Self: Sized {
        // println!("data: {:?}", bytes);
        //
        let data = truncate_packet_header(bytes).await?;

        // println!("data: {:?}", data);

        let string_length = data[0] as usize;

        // println!("string_length: {:?}", string_length);

        let username = String::from_utf8(data[1..string_length + 1].to_vec())?;

        // println!("username: {:?}", username);



        Ok(Self {
            username,
        })
    }

    fn get_id(&self) -> u32 {
        0x00
    }

    async fn handle(&self, stream: &mut Connection) {
        // println!("data: {:?}", self.username);

        let success_packet = PacketPlayOutLoginSuccess {
            username: self.username.clone(),
        } ;

        let serialized = PacketPlayOutLoginSuccess::serialize(&success_packet).await.unwrap();

        stream.send_packet_bytes(serialized).await.unwrap();
    }
}