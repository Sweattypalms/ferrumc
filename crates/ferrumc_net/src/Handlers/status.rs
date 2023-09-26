use crate::{err::FerrumcError, int_to_varint, varint_to_int, Connection, ConnectionState};
use std::io::Cursor;

use serde::Serialize;

use log::{error, trace};

pub async fn status(connection: &mut Connection) -> Result<Option<Vec<u8>>, FerrumcError> {
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
            text: "FerrumC - A Minecraft Server".to_string(),
        },
    };

    let json_bytes = serde_json::to_vec(&payload).expect("Unable to serialize JSON");

    let mut temp_buffer = vec![];

    // Write Packet ID (0x00)
    temp_buffer.push(0x00);

    // Write the length of the JSON string as VarInt
    temp_buffer.extend_from_slice(&int_to_varint(json_bytes.len() as u32));

    // Write JSON string bytes
    temp_buffer.extend_from_slice(&*json_bytes);

    let packet_length = temp_buffer.len() as i32;

    let mut final_buffer = vec![];

    // Write the total packet length as VarInt
    final_buffer.extend_from_slice(&int_to_varint(packet_length as u32));

    final_buffer.extend_from_slice((&temp_buffer).as_ref());

    Ok(Some(final_buffer))
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