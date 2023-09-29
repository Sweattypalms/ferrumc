use std::io::Cursor;
use base64::{Engine};
use image::load_from_memory;
use lazy_static::lazy_static;
use log::{debug, error};
use serde_derive::Serialize;
use ferrumc::create_packet;
use crate::config::CONFIG;
use crate::err::FerrumcError;
use crate::server::connection::Connection;
use crate::utils::MinecraftWriterExt;


pub async fn status(connection: &mut Connection) -> Result<(), FerrumcError> {
    let mut sample = Vec::new();
    let sample_player = Sample {
        name: "§9§lFerrumC".to_string(),
        id: "2b3414ed-468a-45c2-b113-6c5f47430edc".to_string()
    };
    sample.push(sample_player);

    let config = CONFIG.clone();
    let payload = JsonResponse {
        version: Version {
            name: "FerrumC - 1.17.1".to_string(),
            protocol: 756,
        },
        players: Players {
            max: config.max_players,
            online: 0,
            sample,
        },
        description: Description {
            text: config.motd,
        },
        favicon: ICON_BASE64.clone(),
    };

    let mut buffer = Vec::new();

    let mut payload = serde_json::to_vec(&payload)?;
    buffer.write_varint(payload.len() as i32)?;
    buffer.append(&mut payload);

    let payload = create_packet!(0x00, buffer)?;
    connection.write(&payload).await?;
    Ok(())
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

    base64::engine::general_purpose::STANDARD.encode(buf.get_ref())
}