#![allow(clippy::struct_field_names)]
use crate::Result as CustomResult;
use core::fmt;
use std::error::Error;

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

pub const PNG_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = generate_crc(&chunk_type.bytes(), &data);
        Chunk {
            length: u32::try_from(data.len()).unwrap(),
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data_as_string(&self) -> CustomResult<String> {
        Ok(String::from_utf8(self.data().to_owned())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            return Err("Buffer too short".into());
        }

        let (length, chunk_type, data, crc) = parse_chunk_bytes(value)?;

        if !check_crc(&chunk_type.bytes(), &data, crc) {
            return Err(format!(
                "CRC incorrect: expected {}, got {}",
                crc,
                generate_crc(&chunk_type.bytes(), &data)
            )
            .into());
        }

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

/// Helper function to generate a crc from the bytes and data of the chunk
fn generate_crc(chunk_bytes: &[u8], data: &[u8]) -> u32 {
    let mut digest = PNG_CRC.digest();
    digest.update(chunk_bytes);
    digest.update(data);
    digest.finalize()
}

/// Helper function to check that a crc is valid
fn check_crc(chunk_bytes: &[u8], data: &Vec<u8>, crc_to_check: u32) -> bool {
    let combined_bytes: Vec<u8> = chunk_bytes.iter().chain(data).copied().collect();
    let calculated_crc = PNG_CRC.checksum(&combined_bytes);
    calculated_crc == crc_to_check
}

type ParsedChunkResultType = Result<(u32, ChunkType, Vec<u8>, u32), Box<dyn Error>>;

/// Helper function to parse an input vec into the data we need for a `Chunk`
fn parse_chunk_bytes(value: &[u8]) -> ParsedChunkResultType {
    if value.len() < 12 {
        return Err("Buffer too short".into());
    }

    let (data_length_bytes, rest) = value.split_at(4);
    let length = u32::from_be_bytes(data_length_bytes.try_into()?);

    let (chunk_type_bytes, rest) = rest.split_at(4);
    let bytes: [u8; 4] = chunk_type_bytes.try_into()?;
    let chunk_type = ChunkType::try_from(bytes)?;

    if !chunk_type.is_valid() {
        return Err("Invalid chunk type".into());
    }

    let (message_bytes, rest) = rest.split_at(length as usize);

    let (crc_bytes, _) = rest.split_at(4);
    let crc = u32::from_be_bytes(crc_bytes.try_into()?);

    Ok((length, chunk_type, message_bytes.to_vec(), crc))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
