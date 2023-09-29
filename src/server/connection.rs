use std::io::Cursor;
use log::{trace};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::err::FerrumcError;
use crate::server::packet::handle_packet;
use crate::utils::MinecraftReaderExt;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}

#[derive(Debug)]
pub struct Connection {
    pub state: ConnectionState,
    pub stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            state: ConnectionState::Handshaking,
            stream,
        }
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<(), FerrumcError> {
        self.stream.write_all(&data).await?;
        Ok(())
    }
    pub async fn close(&mut self) {
        if let Err(_) = self.stream.shutdown().await {
            trace!("Error closing connection of addr: {}", self.stream.peer_addr().unwrap());
        }
    }
    pub async fn start_connection(&mut self) -> Result<(), FerrumcError> {
        loop {
            let mut length_buffer = [0u8; 1]; // 1 byte, can have at most 255 bytes in a packet
            let n = self.stream.read(&mut length_buffer).await?;

            if n == 0 {
                return Ok(());
            }

            let length = length_buffer[0] as usize;

            // trace!("Packet length: {}", length);

            let mut packet_buffer = vec![0u8; length];
            self.stream.read_exact(&mut packet_buffer).await?;

            // trace!("Packet: {:?}", packet_buffer);

            let mut cursor = Cursor::new(packet_buffer);
            let packet_id = cursor.read_varint()?;

            // trace!("Packet ID: {}", packet_id);

            let mut buf = cursor.into_inner();
            // remove the packet id from the buffer
            buf[0..].rotate_left(1);
            // trace!("Packet buffer: {:?}", buf);

            // let mut packet = PACKET_REGISTRY.get_packet(self.state, packet_id as u8, buf).await?;
            // packet.handle(self).await?;
            // handle_packet(self, packet_id as u8, buf).await?;
            match handle_packet(self, packet_id as u8, buf).await {
                Ok(_) => {},
                Err(err) => {
                    trace!("Packet error: {}", err);
                }
            }
        }
    }
}