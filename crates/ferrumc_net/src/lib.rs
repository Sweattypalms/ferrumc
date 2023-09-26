use log::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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
        let mut inbound_packet = vec![0u8; 256];
        let read_bytes = connection
            .stream
            .read(&mut inbound_packet)
            .await
            .expect("Unable to read packet");
        if read_bytes == 0 {
            info!("Connection closed");
            break;
        }
        info!("Read {} bytes", read_bytes);
        info!(
            "Raw Data: {} ",
            parse_packet_string(&inbound_packet[0..read_bytes])
        );
        let return_data =
            dispatcher::dispatch(&mut connection, inbound_packet[0..read_bytes].to_vec())
                .await
                .expect("Unable to dispatch packet");
        if let Some(data) = return_data {
            trace!("Sending {:?} bytes of data", data.len());
            connection
                .stream
                .write_all(&data)
                .await
                .expect("Unable to write packet");
            connection.stream.flush().await.unwrap();
        } else {
            trace!("No data to send");
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