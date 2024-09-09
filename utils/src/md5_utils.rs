use md5;
use crate::base64::vec_to_base64;

pub fn md5_digest(s: &str) -> String {
    let digest = md5::compute(s.as_bytes());
    vec_to_base64(digest.to_vec())
}