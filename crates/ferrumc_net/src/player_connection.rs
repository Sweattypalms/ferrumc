use crate::packet::handle_packet;
use ferrumc_utils::err::FerrumcError;
use ferrumc_utils::utils::MinecraftReaderExt;
use ferrumc_utils::utils::MinecraftWriterExt;
use log::trace;
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::channel;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}

#[derive(Debug)]
pub(crate) struct ClientSettings {
    pub(crate) locale: String,
    pub view_distance: i32,
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub displayed_skin_parts: i32,
    pub main_hand: i32,
    pub disable_text_filtering: bool,
}

#[derive(Debug)]
pub struct Connection {
    pub state: ConnectionState,
    pub stream: TcpStream,
    pub username: Option<String>,
    pub uuid: Option<Uuid>,
    pub client_settings: Option<ClientSettings>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            state: ConnectionState::Handshaking,
            stream,
            username: None,
            uuid: None,
            client_settings: None,
        }
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<(), FerrumcError> {
        self.stream.write_all(&data).await?;
        Ok(())
    }
    pub async fn close(&mut self) {
        if let Err(_) = self.stream.shutdown().await {
            trace!(
                "Error closing connection of addr: {}",
                self.stream.peer_addr().unwrap()
            );
        }
    }
    // pub async fn start_connection(&mut self) -> Result<(), FerrumcError> {
    //     let (tick_tx, mut tick_rx) = channel(10);
    //
    //     tokio::spawn(async move {
    //         let mut tick_timer = tokio::time::interval(std::time::Duration::from_millis(50));
    //         loop {
    //             tick_timer.tick().await;
    //
    //
    //
    //             if tick_tx.send(()).await.is_err() {
    //                 trace!("Tick receiver dropped");
    //                 return;
    //             }
    //         }
    //     });
    //
    //     loop {
    //         let mut length_buffer = [0u8; 1]; // 1 byte, can have at most 255 bytes in a packet
    //         let n = self.stream.read(&mut length_buffer).await?;
    //
    //         if n == 0 {
    //             return Ok(());
    //         }
    //
    //         let length = length_buffer[0] as usize;
    //
    //         // trace!("Packet length: {}", length);
    //
    //         let mut packet_buffer = vec![0u8; length];
    //         self.stream.read_exact(&mut packet_buffer).await?;
    //
    //         // trace!("Packet: {:?}", packet_buffer);
    //
    //         let mut cursor = Cursor::new(packet_buffer);
    //         let packet_id = cursor.read_varint()?;
    //
    //         // trace!("Packet ID: {}", packet_id);
    //
    //         let mut buf = cursor.into_inner();
    //         // remove the packet id from the buffer
    //         buf[0..].rotate_left(1);
    //         // trace!("Packet buffer: {:?}", buf);
    //
    //         match handle_packet(self, packet_id as u8, buf).await {
    //             Ok(_) => {}
    //             Err(err) => {
    //                 trace!("Packet error: {:?}", err);
    //             }
    //         }
    //     }
    // }

    pub async fn start_connection(&mut self) -> Result<(), FerrumcError> {
        let (tick_tx, mut tick_rx) = tokio::sync::mpsc::channel(10);

        tokio::spawn(async move {
            let mut tick_timer = tokio::time::interval(std::time::Duration::from_millis(50));
            loop {
                tick_timer.tick().await;
                if tick_tx.send(()).await.is_err() {
                    trace!("Tick receiver dropped");
                    return;
                }
            }
        });

        loop {
            tokio::select! {
                _ = tick_rx.recv() => {
                    if let Err(e) = self.tick().await {
                        trace!("Error in tick: {:?}", e);
                    }
                }
                result = self.read_packet() => {
                    if let Err(e) = result {
                        trace!("Error reading/handling packet: {:?}", e);
                    }
                }
            }
        }
    }

    pub async fn read_packet(&mut self) -> Result<(), FerrumcError> {
        let mut length_buffer = [0u8; 1]; // 1 byte, can have at most 255 bytes in a packet
        let n = self.stream.read(&mut length_buffer).await?;

        if n == 0 {
            return Ok(());
        }

        let length = length_buffer[0] as usize;
        let mut packet_buffer = vec![0u8; length];
        self.stream.read_exact(&mut packet_buffer).await?;

        let mut cursor = Cursor::new(packet_buffer);
        let packet_id = cursor.read_varint()?;

        let mut buf = cursor.into_inner();
        buf[0..].rotate_left(1);

        match handle_packet(self, packet_id as u8, buf).await {
            Ok(_) => {}
            Err(err) => {
                trace!("Packet error: {:?}", err);
            }
        }

        Ok(())
    }

    pub async fn tick(&mut self) -> Result<(), FerrumcError> {
        if self.username.is_none() || self.stream.peer_addr().is_err() {
            return Ok(());
        }

        // let mut buffer = Vec::new();
        //
        // // write long
        // buffer.write_i64(69420).await?;
        //
        // let raw = crate::create_packet!(0x23, buffer)?;
        //
        // self.write(&raw).await?;
        //
        // trace!("Sent keep alive packet");

        //trace!("Player ticked.");

        Ok(())
    }
}