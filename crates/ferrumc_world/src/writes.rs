use crate::nbtstructs::{Chunk, PaletteBlock, Section};

/// This function gets a 64 bit integer and splits it into 12 5 bit integers.
/// Since we don't have actual 5 bit integers, we use u8s instead.
/// The first u8 is the last 5 bits of the long, the second is the next 5 bits, etc.
pub fn long_to_5bit_array(long: i64) -> [u8; 12] {
    let mut array: [u8; 12] = [0; 12];
    for i in 0..12 {
        array[i] = ((long >> (i * 5)) & 0b11111) as u8;
    }
    array
}

/// This function gets a 12 element array of 5 bit integers and combines them into a 64 bit integer.
/// The first u8 is the last 5 bits of the long, the second is the next 5 bits, etc.
pub fn long_from_5bit_array(array: [u8; 12]) -> i64 {
    let mut long: i64 = 0;
    for i in 0..12 {
        long |= (array[i] as i64) << (i * 5);
    }
    long
}

pub fn long_array_to_5bit_array(longs: &[i64]) -> Vec<u8> {
    let mut array: Vec<u8> = Vec::new();
    for long in longs {
        array.extend_from_slice(&long_to_5bit_array(*long));
    }
    if array.len() % 12 != 0 {
        panic!("Array length must be a multiple of 12");
    }
    array
}