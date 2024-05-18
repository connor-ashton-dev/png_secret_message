use core::fmt;
use std::convert::TryInto;
use std::str::{from_utf8, FromStr, Utf8Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Ancillary bit: bit 5 of first byte
    /// 0 (uppercase) = critical, 1 (lowercase) = ancillary
    #[allow(dead_code)]
    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    /// Private bit: bit 5 of second byte
    /// 0 (uppercase) = public, 1 (lowercase) = private
    #[allow(dead_code)]
    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    /// Reserved bit: bit 5 of third byte
    /// Must be 0 (uppercase)
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    /// Safe-to-copy bit: bit 5 of fourth byte
    /// 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy
    #[allow(dead_code)]
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }

    /// Checks to see if each byte is valid with `is_valid_byte`
    pub fn is_valid(&self) -> bool {
        // check length
        if self.bytes.len() != 4 {
            return false;
        }

        // check alphabetical
        for byte in &self.bytes {
            if !is_valid_byte(*byte) {
                return false;
            }
        }

        // check reserved bit
        self.is_reserved_bit_valid()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType { bytes: value };
        if chunk_type.is_valid() {
            Ok(chunk_type)
        } else {
            Err("Error occured")
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = str_to_bytes(s)?;
        Ok(Self { bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match bytes_to_str(&self.bytes) {
            Ok(str) => write!(f, "{str}"),
            Err(e) => write!(f, "{e}"),
        }
    }
}

/// Checks to see if byte is A-Z or a-z
pub fn is_valid_byte(byte: u8) -> bool {
    byte.is_ascii_alphabetic()
}

/// tries to convert a &str to a [u8;4] to put it into the bytes
fn str_to_bytes(s: &str) -> Result<[u8; 4], &'static str> {
    let bytes: Result<[u8; 4], _> = s.as_bytes().try_into();
    match bytes {
        Ok(b) if b.iter().all(|&byte| byte.is_ascii_alphabetic()) => Ok(b),
        Ok(_) => Err("String contains non-alphabetical characters"),
        Err(_) => Err("String must be exactly 4 characters long"),
    }
}

fn bytes_to_str(bytes: &[u8]) -> Result<&str, Utf8Error> {
    from_utf8(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
