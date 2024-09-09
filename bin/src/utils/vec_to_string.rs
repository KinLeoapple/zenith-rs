use std::str;

#[derive(Debug)]
pub enum StringConversionError {
    InvalidUTF8Sequence
}

pub fn vec_to_string(vec: Vec<u8>) -> Result<String, StringConversionError> {
    match str::from_utf8(vec.as_slice()) {
        Ok(v) => Ok(v.to_string()),
        Err(_e) => Err(StringConversionError::InvalidUTF8Sequence),
    }
}