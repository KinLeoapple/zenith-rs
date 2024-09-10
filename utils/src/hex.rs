use hex::{decode, encode, encode_to_slice, FromHexError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HexString {
    pub hex: String,
}

pub fn to_hex(bytes: &[u8]) -> HexString {
    HexString { hex: encode(bytes) }
}

pub fn to_hex_slice(bytes: &[u8]) -> Result<Vec<u8>, FromHexError> {
    let mut vec: Vec<u8> = Vec::new();
    match encode_to_slice(bytes, &mut vec) {
        Ok(_) => Ok(vec),
        Err(e) => Err(e),
    }
}

pub fn to_vec(hex: &str) -> Result<Vec<u8>, FromHexError> {
    match decode(hex) {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}