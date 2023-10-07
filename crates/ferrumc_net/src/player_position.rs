use crate::packet::PacketData;
use byteorder::{BigEndian, ReadBytesExt};
use ferrumc_utils::err::FerrumcError;
use log::trace;
use std::io::Cursor;

pub async fn player_position(packet_data: &mut PacketData<'_>) -> Result<(), FerrumcError> {
    let mut cursor = Cursor::new(&packet_data.bytes);

    // trace!("data: {:?}", packet_data.bytes);

    let _x = cursor.read_f64::<BigEndian>()?;
    let _y = cursor.read_f64::<BigEndian>()?;
    let _z = cursor.read_f64::<BigEndian>()?;
    let _on_ground = cursor.read_u8()? != 0;

    // log::trace!("x: {}, y: {}, z: {}, on_ground: {}", x, y, z, on_ground);

    Ok(())
}

pub async fn set_on_ground(packet_data: &mut PacketData<'_>) -> Result<(), FerrumcError> {
    let mut cursor = Cursor::new(&packet_data.bytes);

    let on_ground = cursor.read_u8()? != 0;
    trace!("on_ground: {}", on_ground);

    Ok(())
}

pub async fn player_rotation(packet_data: &mut PacketData<'_>) -> Result<(), FerrumcError> {
    let mut cursor = Cursor::new(&packet_data.bytes);

    let yaw = cursor.read_f32::<BigEndian>()?;
    let pitch = cursor.read_f32::<BigEndian>()?;
    let on_ground = cursor.read_u8()? != 0;

    Ok(())
}

pub async fn player_position_and_rotation(
    packet_data: &mut PacketData<'_>,
) -> Result<(), FerrumcError> {
    let mut cursor = Cursor::new(&packet_data.bytes);

    let x = cursor.read_f64::<BigEndian>()?;
    let y = cursor.read_f64::<BigEndian>()?;
    let z = cursor.read_f64::<BigEndian>()?;
    let yaw = cursor.read_f32::<BigEndian>()?;
    let pitch = cursor.read_f32::<BigEndian>()?;
    let on_ground = cursor.read_u8()? != 0;

    Ok(())
}