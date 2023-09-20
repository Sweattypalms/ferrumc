use std::io::{Cursor, Read, Write};
use anyhow::Error;
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use tokio::io::AsyncReadExt;
use crate::network::packet::{InboundPacket, OutboundPacket};



pub fn read_varlong<R: Read>(reader: &mut R) -> Result<i64> {
    // works fine
    reader.read_i64::<BigEndian>().map_err(Error::from)
}

// Works fine
pub async fn write_varlong<W: Write>(writer: &mut W, value: i64) -> Result<()> {
    writer.write_i64::<BigEndian>(value).map_err(Error::from)
}


// used in read_varint implemenetation
const LAST_SEVEN_BITS: i32 = 0b0111_1111;
const NEXT_BYTE_EXISTS: u8 = 0b1000_0000;

// bit mask to remove remaining 7  MSB's after right shift
const SEVEN_BITS_SHIFT_MASK: i32 = 0x01_ff_ff_ff;

pub(crate) async fn write_varint(sink: &mut Vec<u8>, mut value: i32) {
    loop {
        let mut temp = (value & LAST_SEVEN_BITS) as u8;
        // i32 right shift is arithmetic shift (preserves MSB)
        value >>= 7;
        value &= SEVEN_BITS_SHIFT_MASK;
        if value != 0 {
            temp |= NEXT_BYTE_EXISTS;
        }
        sink.push(temp);
        if value == 0 {
            break;
        }
    }
}

pub fn read_varint<R: Read>(mut reader: R) -> Result<i32> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read = 0x80; // Dummy value to start the loop

    while (read & 0x80) != 0 {
        read = reader.read_u8()?; // Read one byte
        let val = read & 0x7F; // Take the last 7 bits of the byte
        result |= (val as i32) << (7 * num_read); // Shift the 7 bits to their proper place

        num_read += 1;

        if num_read > 5 {
            return Err(Error::msg("VarInt is too large"));
        }
    }

    Ok(result)
}
pub async fn read_varint_async(stream: &mut tokio::net::TcpStream) -> Result<i32, anyhow::Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut tmp = [0u8; 1];

    // Read bytes one by one until a valid varint is read into the buffer
    loop {
        stream.read_exact(&mut tmp).await?;

        println!("tmp: {:?}", tmp);

        buffer.push(tmp[0]);
        if (tmp[0] & 0x80) == 0 {
            break;
        }
    }

    // Use your existing read_varint function to parse the varint
    let mut cursor = std::io::Cursor::new(buffer);
    read_varint(&mut cursor)
}

//
// /**
//     * Needs fixing.
//     * Serialize a packet with a string.
//     * The string is serialized as a VarInt containing the length of the string, followed by the actual string data.
//  */
pub async fn serialize_packet_with_str(packet: Box<dyn OutboundPacket>, string: String) -> Result<Vec<u8>> {
    let packet_id = packet.get_id();

    let mut temp_buffer = Vec::new();

    // Write packet_id
    write_varint(&mut temp_buffer, packet_id as i32).await;

    // Write string length
    write_varint(&mut temp_buffer, string.len() as i32).await;

    // Write string data
    temp_buffer.extend_from_slice(string.as_bytes());

    let mut final_buffer = Vec::new();

    // Write the total packet length as VarInt

    write_varint(&mut final_buffer, temp_buffer.len() as i32).await;

    final_buffer.extend_from_slice(&temp_buffer);

    Ok(final_buffer)
}


/// Used for constructing packets from bytes. Only used in initializing the packet registry.
pub async fn construct_async<T: InboundPacket + Send + 'static>(bytes: Vec<u8>) -> Result<Box<dyn InboundPacket>, Error> {
    let packet = T::deserialize(bytes).await?;
    Ok(Box::new(packet))
}

/// Truncates the packet length and packet ID VarInts from the beginning of the packet bytes.
/// Returns a new vector containing the truncated bytes.
pub async fn truncate_packet_header(bytes: Vec<u8>) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(bytes);

    // Read the VarInt for packet length (and do something with it if needed)
    let _packet_length = read_varint(&mut cursor)?;

    // Read the VarInt for packet ID (and do something with it if needed)
    let _packet_id = read_varint(&mut cursor)?;

    // Create a new byte vector containing the remaining bytes after the two VarInts
    let truncated_bytes = cursor.clone().into_inner()[cursor.position() as usize..].to_vec();

    Ok(truncated_bytes)
}