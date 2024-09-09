use std::str;
use crate::utils::error::{Error, StringConversionError};

pub fn vec_to_string(vec: Vec<u8>) -> Result<String, Error> {
    match str::from_utf8(vec.as_slice()) {
        Ok(v) => Ok(v.to_string()),
        Err(_e) => Err(StringConversionError::InvalidUTF8Sequence.error()),
    }
}