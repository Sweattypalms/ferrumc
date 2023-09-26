use crate::err::FerrumcError;
use crate::Connection;

use log::info;

pub async fn login(
    packet: Vec<u8>,
    connection: &mut Connection,
) -> Result<Option<Vec<u8>>, FerrumcError> {
    let packet_length = packet[0];
    let packet_id = packet[1];
    let username = String::from_utf8(packet[3..=(packet_length) as usize].to_vec())
        .expect("Unable to parse username");
    info!("Username: {}", username);
    Ok(None)
}