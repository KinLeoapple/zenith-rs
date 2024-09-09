use md5;
use crate::utils::vec_to_string::{vec_to_string, StringConversionError};

pub fn md5_digest(s: &str) -> Result<String, StringConversionError> {
    let digest = md5::compute(s.as_bytes());
    vec_to_string(digest.to_vec())
}