use log::trace;
use crate::err::FerrumcError;
use crate::server::packet::PacketData;
use crate::utils::{MinecraftWriterExt};

pub async fn ping(packet_data: PacketData<'_>) -> Result<(), FerrumcError> {

    trace!("Ping packet received");

    let mut buffer = vec![];
    buffer.write_varint(0x09)?; // 0x08 => Long
    buffer.write_varint(0x01)?;
    buffer.extend_from_slice(&packet_data.bytes);

    packet_data.connection.write(&buffer).await?;

    packet_data.connection.close().await;

    Ok(())
}