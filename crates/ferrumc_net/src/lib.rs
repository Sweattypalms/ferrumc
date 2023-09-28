use std::io::Read;
use log::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::err::FerrumcError;
use byteorder::{BigEndian, ReadBytesExt};

mod Handlers;
mod dispatcher;
mod err;

#[derive(Debug)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}

pub struct Connection {
    pub state: ConnectionState,
    pub stream: TcpStream,
    pub player_uuid: String,
}

fn parse_packet_string(packet: &[u8]) -> String {
    let mut string = String::new();
    let mut buffer = Vec::new();
    for byte in packet {
        // If the byte is a printable ASCII character, add it to the string
        if 0x20 <= *byte && *byte <= 0x7e {
            buffer.push(*byte);
            string += &String::from_utf8(buffer.clone()).unwrap();
            buffer.clear();
        } else {
            buffer.push(b'*');
        }
    }
    string
}

pub async fn handle_connection(stream: TcpStream) {
    let mut connection = Connection {
        state: ConnectionState::Handshaking,
        stream,
        player_uuid: "".to_string(),
    };

    loop {
        // let mut inbound_packet = vec![0u8; 256];
        // let read_bytes = connection
        //     .stream
        //     .read(&mut inbound_packet)
        //     .await
        //     .expect("Unable to read packet");
        // if read_bytes == 0 {
        //     info!("Connection closed");
        //     break;
        // }

        let mut length_buf = [0u8; 1];


        info!("Read {} bytes", read_bytes);
        info!(
            "Raw Data: {} ",
            parse_packet_string(&inbound_packet[0..read_bytes])
        );
        let result = dispatcher::dispatch(&mut connection, inbound_packet[0..read_bytes].to_vec()).await;

        match result {
            Ok(Some(data)) => {
                trace!("Sending {:?} bytes of data", data.len());
                connection.stream.write_all(&data).await.expect("Unable to write packet");
                connection.stream.flush().await.unwrap();
            }
            Ok(None) => {
                trace!("No data to send");
            }
            Err(FerrumcError::InvalidPacketID) => {
                warn!("Received an unknown packet ID");
            }
            Err(e) => {
                error!("Error handling packet: {:?}", e);
            }
        }
        inbound_packet.clear();
    }
}

pub fn varint_to_int(packet: &[u8]) -> i32 {
    let mut result = 0;
    let mut num_read = 0;
    for byte in packet {
        let value = *byte & 0b01111111;
        result |= (value as i32) << (7 * num_read);
        num_read += 1;
        if num_read > 5 {
            panic!("VarInt is too big");
        }
        if (*byte & 0b10000000) == 0 {
            break;
        }
    }
    result
}

pub fn int_to_varint(mut value: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let mut temp = (value & 0b01111111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }
        bytes.push(temp);
        if value == 0 {
            break;
        }
    }
    bytes
}

pub fn read_varint<R: Read>(mut reader: R) -> Result<i32, FerrumcError> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read = 0x80; // Dummy value to start the loop

    while (read & 0x80) != 0 {
        // read = reader.read_u8()?;
        read = byteorder::ReadBytesExt::read_u8(&mut reader)
            .map_err(|_| FerrumcError::InvalidPacketID)?;
            // .await
            // .map_err(|_| FerrumcError::InvalidPacketID)?; // Read a byte from the stream
        let val = read & 0x7F; // Take the last 7 bits of the byte
        result |= (val as i32) << (7 * num_read); // Shift the 7 bits to their proper place

        num_read += 1;

        if num_read > 5 {
            return Err(FerrumcError::InvalidPacketID);
        }
    }

    Ok(result)
}


trait CursorExt {
    fn read_varint(&mut self) -> Result<i32, FerrumcError>;
    fn read_varstring(&mut self) -> Result<String, FerrumcError>;
    fn read_u16_be(&mut self) -> Result<u16, FerrumcError>;
}

impl<R: Read> CursorExt for R {
    fn read_varint(&mut self) -> Result<i32, FerrumcError> {
        let mut num_read = 0;
        let mut result = 0;
        let mut read = 0x80; // Dummy value to start the loop

        while (read & 0x80) != 0 {
            // read = reader.read_u8()?;
            read = byteorder::ReadBytesExt::read_u8(self)
                .map_err(|_| FerrumcError::InvalidVarInt)?;
            let val = read & 0x7F; // Take the last 7 bits of the byte
            result |= (val as i32) << (7 * num_read); // Shift the 7 bits to their proper place

            num_read += 1;

            if num_read > 5 {
                return Err(FerrumcError::InvalidVarInt);
            }
        }

        Ok(result)
    }

    fn read_varstring(&mut self) -> Result<String, FerrumcError> {
        let length = self.read_varint()?;
        let mut string = vec![0u8; length as usize];
        self.read_exact(&mut string).map_err(|_| FerrumcError::InvalidString)?;
        let string = String::from_utf8(string).map_err(|_| FerrumcError::InvalidString)?;
        Ok(string)
    }

    fn read_u16_be(&mut self) -> Result<u16, FerrumcError> {
        self.read_u16::<BigEndian>().map_err(|_| FerrumcError::InvalidBigEndian)
    }
}