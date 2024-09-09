use base64::{engine::general_purpose, DecodeError, Engine as _};

pub fn vec_to_base64(vec: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(&vec)
}

pub fn base64_to_vec(base64: String) -> Result<Vec<u8>, DecodeError> {
    match general_purpose::STANDARD.decode(&base64) {
        Ok(vec) => Ok(vec),
        Err(e) => Err(e),
    }
}