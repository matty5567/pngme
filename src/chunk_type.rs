#![allow(unused_variables)]

use std::{str::FromStr, str::from_utf8, fmt::Display};


#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    data: u32
}

impl ChunkType {
    pub fn bytes(&self) -> [u8;4] {
        self.data.to_be_bytes()
    }

    pub fn is_valid(&self) -> bool {
        let chars = &self.data.to_be_bytes();
        
        for char in chars {
            if !(char.is_ascii_lowercase() || char.is_ascii_uppercase()) {
                return false;
            }
        };
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        match self.data >> 29 & 1 {
            0 => true,
            _ => false,
        }
    }

    pub fn is_public(&self) -> bool {
        match self.data >> 21 & 1 {
            0 => true,
            _ => false,
        }
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        match self.data >> 13 & 1 {
            0 => true,
            _ => false,
        }
    }

    pub fn is_safe_to_copy(&self) -> bool {
        match self.data >> 5 & 1 {
            1 => true,
            _ => false,
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let data = ((value[0] as u32) << 24) +
            ((value[1] as u32) << 16 )+
            ((value[2] as u32) <<  8 )+
            ((value[3] as u32) <<  0 );

        return Result::Ok(ChunkType {data})
    }
}

impl FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.bytes().len() > 4  || !s.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_uppercase()) {
            return Result::Err(String::from("String must 4 ascii alphabetic chars"));
        } else{
            let bytes: [u8; 4] = s.as_bytes().try_into().unwrap();
            if let Ok(chunk_type) = ChunkType::try_from(bytes) {
                return Result::Ok(chunk_type)
            } else {
                return Result::Err(String::from("failed to convert to bytes"))
            }
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.data.to_be_bytes()).unwrap())
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

