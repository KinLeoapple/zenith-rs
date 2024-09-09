use crate::utils::error::Error;
use crate::utils::vec_to_string::vec_to_string;
use md5;

pub fn md5_digest(s: &str) -> Result<String, Error> {
    let digest = md5::compute(s.as_bytes());
    vec_to_string(digest.to_vec())
}