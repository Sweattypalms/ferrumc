use crate::{err::FerrumcError, int_to_varint, varint_to_int, Connection, ConnectionState};
use std::io::Cursor;
use base64::encode;
use image::load_from_memory;
use lazy_static::lazy_static;

use serde::Serialize;

use log::{debug, error, trace};

pub async fn status(connection: &mut Connection) -> Result<Option<Vec<u8>>, FerrumcError> {
    let mut sample = Vec::new();
    let sample_player = Sample {
        name: "§9§lFerrumC".to_string(),
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
        favicon: ICON_BASE64.clone(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    favicon: Option<String>
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

lazy_static! {
    static ref ICON_BASE64: Option<String> = {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
            debug!("Failed to get CARGO_MANIFEST_DIR, using current directory instead.");
            ".".to_string()
        });

        let img_path = format!("{}/icon-64.png", root);
        match std::fs::read(&img_path) {
            Ok(bytes) => {
                if bytes.is_empty() {
                    error!("Image file is empty: {}", img_path);
                    None
                } else {
                    Some(format!("data:image/png;base64,{}", png_to_base64(&bytes)))
                }
            },
            Err(_) => {
                error!("Failed to read the image file: {}", img_path);
                None
            }
        }
    };
}

fn png_to_base64(png_bytes: &[u8]) -> String {
    let img = load_from_memory(&png_bytes).unwrap();

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();

    encode(buf.get_ref())
}

