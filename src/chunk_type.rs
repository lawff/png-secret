use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChunkTypeError {
    #[error("ChunkTypeError Bad byte: {0}")]
    BadByte(u8),

    #[error("ChunkTypeError Bad length for {0} is {1}")]
    BadLength(String, usize),
}

/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    data: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.data
    }

    pub fn is_critical(&self) -> bool {
        Self::bit_is_zero(self.bytes()[0], 5)
    }

    pub fn is_public(&self) -> bool {
        Self::bit_is_zero(self.bytes()[1], 5)
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        Self::bit_is_zero(self.bytes()[2], 5)
    }

    pub fn is_safe_to_copy(&self) -> bool {
        !Self::bit_is_zero(self.bytes()[3], 5)
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
            && Self::is_valid_byte(self.bytes()[0])
            && Self::is_valid_byte(self.bytes()[1])
            && Self::is_valid_byte(self.bytes()[2])
            && Self::is_valid_byte(self.bytes()[3])
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }

    #[doc(hidden)]
    fn bit_is_zero(bit: u8, n: u8) -> bool {
        let mask = 1 << n;
        bit & mask == 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in bytes {
            if !Self::is_valid_byte(byte) {
                return Err(ChunkTypeError::BadByte(byte));
            }
        }

        Ok(ChunkType { data: bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in self.data {
            write!(f, "{}", b as char)?;
        }

        Ok(())
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(ChunkTypeError::BadLength(s.to_string(), s.len()));
        }

        let mut ret: [u8; 4] = [0; 4];
        for (i, b) in s.as_bytes().iter().enumerate() {
            if Self::is_valid_byte(*b) {
                ret[i] = *b;
            } else {
                return Err(ChunkTypeError::BadByte(*b));
            }
        }

        Ok(ChunkType { data: ret })
    }
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
