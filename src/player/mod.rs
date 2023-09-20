use std::io::Cursor;
use std::sync::Arc;

use anyhow::{Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::network::connection::state::ConnectionState;
use crate::network::packet::{OutboundPacket, REGISTRY};


pub struct Player {
    pub uuid: String,
    pub username: String,
    pub connection: Arc<Mutex<Connection>>,  // Wrapped in Mutex for mutable access
}

impl Player {
    pub fn new(stream: TcpStream) -> Self {
        Player {
            uuid: "".to_string(),
            username: "".to_string(),
            connection: Arc::new(Mutex::new(Connection::new(ConnectionState::Handshaking, stream))),
        }
    }

    pub async fn init(&self) -> Result<()> {
        let mut connection = self.connection.lock().await;
        connection.start_connection().await?;
        Ok(())
    }
}


pub struct Connection {
    pub state: ConnectionState,
    pub stream: TcpStream,
}

impl Connection {
    pub fn new(state: ConnectionState, stream: TcpStream) -> Self {
        Connection {
            state,
            stream,
        }
    }
    pub async fn send_packet(&mut self, packet: impl OutboundPacket) -> Result<()> {
        let serialized = packet.serialize().await?;

        self.stream.write_all(serialized.as_ref()).await?;

        Ok(())
    }

    pub async fn send_packet_bytes(&mut self, packet: Vec<u8>) -> Result<()> {

        let max_size = 32;

        let mut cursor = Cursor::new(&packet);

        let mut buffer = [0u8; 32];

        let mut bytes_read = 0;

        while bytes_read < packet.len() {
            let bytes_to_read = std::cmp::min(max_size, packet.len() - bytes_read);
            cursor.read_exact(&mut buffer[..bytes_to_read]).await?;
            self.stream.write_all(&buffer[..bytes_to_read]).await?;
            bytes_read += bytes_to_read;
        }
        Ok(())

    }

    pub async fn start_connection(&mut self) -> Result<()> {
        loop {
            let mut length_buf = [0u8; 1]; // 1-byte buffer to store packet length
            let n = self.stream.read(&mut length_buf).await?;

            if n == 0 {
                return Ok(()); // Connection closed
            }

            let packet_length = length_buf[0] as usize;
            // println!("Packet length byte read: {}", packet_length);

            let mut packet_data = vec![0u8; packet_length + 1];
            packet_data[0] = length_buf[0]; // copy the length byte to the packet data for deserialization
            self.stream.read_exact(&mut packet_data[1..]).await?;
            // println!("Received packet data: {:?}", packet_data);

            if let Some(packet) = REGISTRY.deserialize_inbound(self.state, packet_data).await {
                print!("Packet Id: {:?}", packet.get_id());
                println!("\tCurrent State: {:?}", self.state);
                packet.handle(self).await;
            }
        }
    }
}