use crate::err::FerrumcError;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Read;

pub trait MinecraftReaderExt {
    fn read_varint(&mut self) -> Result<i32, FerrumcError>;
    fn read_varstring(&mut self) -> Result<String, FerrumcError>;
    fn read_u16_be(&mut self) -> Result<u16, FerrumcError>;
}

impl<R: Read> MinecraftReaderExt for R {
    fn read_varint(&mut self) -> Result<i32, FerrumcError> {
        let mut num_read = 0;
        let mut result = 0;
        let mut read = 0x80; // Dummy value to start the loop

        while (read & 0x80) != 0 {
            read = ReadBytesExt::read_u8(self).map_err(|_| FerrumcError::InvalidVarInt)?;
            let val = read & 0x7F; // Take the last 7 bits of the byte
            result |= (val as i32) << (7 * num_read); // Shift the 7 bits to their proper place

            num_read += 1;

            if num_read > 5 {
                return Err(FerrumcError::InvalidVarInt);
            }
        }

        Ok(result)
    }

    fn read_varstring(&mut self) -> Result<String, FerrumcError> {
        let length = self.read_varint()?;
        let mut string = vec![0u8; length as usize];
        self.read_exact(&mut string)
            .map_err(|_| FerrumcError::InvalidString)?;
        let string = String::from_utf8(string).map_err(|_| FerrumcError::InvalidString)?;
        Ok(string)
    }

    fn read_u16_be(&mut self) -> Result<u16, FerrumcError> {
        self.read_u16::<BigEndian>()
            .map_err(|_| FerrumcError::InvalidBigEndian)
    }
}

pub trait MinecraftWriterExt {
    fn write_varint(&mut self, value: i32) -> Result<(), FerrumcError>;
    fn write_varstring(&mut self, value: &str) -> Result<(), FerrumcError>;
    fn write_u16_be(&mut self, value: u16) -> Result<(), FerrumcError>;
}

impl<W: std::io::Write> MinecraftWriterExt for W {
    fn write_varint(&mut self, mut value: i32) -> Result<(), FerrumcError> {
        loop {
            let mut temp = (value & 0b01111111) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0b10000000;
            }
            self.write_all(&[temp])?;
            if value == 0 {
                break;
            }
        }
        Ok(())
    }

    fn write_varstring(&mut self, value: &str) -> Result<(), FerrumcError> {
        self.write_varint(value.len() as i32)?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }

    fn write_u16_be(&mut self, value: u16) -> Result<(), FerrumcError> {
        self.write_u16::<BigEndian>(value)
            .map_err(|_| FerrumcError::InvalidBigEndian)
    }
}
