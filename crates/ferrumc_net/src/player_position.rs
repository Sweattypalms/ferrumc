use crate::packet::PacketData;
use byteorder::{BigEndian, ReadBytesExt};
use ferrumc_utils::err::FerrumcError;
use std::io::Cursor;


pub async fn player_position(packet_data: PacketData<'_>) -> Result<(), FerrumcError> {
    let mut cursor = Cursor::new(&packet_data.bytes);

    // trace!("data: {:?}", packet_data.bytes);

    let _x = cursor.read_f64::<BigEndian>()?;
    let _y = cursor.read_f64::<BigEndian>()?;
    let _z = cursor.read_f64::<BigEndian>()?;
    let _on_ground = cursor.read_u8()? != 0;


    // log::trace!("x: {}, y: {}, z: {}, on_ground: {}", x, y, z, on_ground);

    Ok(())
}