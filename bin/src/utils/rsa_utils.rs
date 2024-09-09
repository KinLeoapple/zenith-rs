use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPublicKey, LineEnding};
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use crate::utils::vec_to_string::vec_to_string;

#[derive(Debug)]
pub enum RSAError {
    RsaEncryptionError,
    RsaDecryptionError,
    RsaKeyToPemError,
    PemToRsaKeyError,
}

pub struct RsaKeyPair {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
}

fn to_string(result: Vec<u8>) -> Result<String, RSAError> {
    match vec_to_string(result) {
        Ok(v) => Ok(v),
        Err(_e) => Err(RSAError::RsaEncryptionError),
    }
}

pub fn generate_key_pair() -> RsaKeyPair {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let pub_key = RsaPublicKey::from(&priv_key);
    RsaKeyPair {
        private_key: priv_key,
        public_key: pub_key,
    }
}
pub fn encrypt(data: &[u8], pub_key: RsaPublicKey) -> Result<String, RSAError> {
    let mut rng = rand::thread_rng();
    let result = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]);
    if result.is_ok() {
        to_string(result.unwrap())
    } else {
        Err(RSAError::RsaEncryptionError)
    }
}

pub fn decrypt(data: &[u8], priv_key: RsaPrivateKey) -> Result<String, RSAError> {
    let result = priv_key.decrypt(Pkcs1v15Encrypt, &data);
    if result.is_ok() {
        to_string(result.unwrap())
    } else {
        Err(RSAError::RsaDecryptionError)
    }
}

pub fn pub_to_pem(pub_key: RsaPublicKey) -> Result<String, RSAError> {
    let result = pub_key.to_pkcs1_pem(LineEnding::CR);
    if result.is_ok() {
        Ok(result.unwrap().to_string())
    } else {
        Err(RSAError::RsaKeyToPemError)
    }
}

pub fn to_pem(priv_key: RsaPrivateKey) -> Result<String, RSAError> {
    let result = priv_key.to_pkcs8_pem(LineEnding::CR);
    if result.is_ok() {
        Ok(result.unwrap().to_string())
    } else {
        Err(RSAError::RsaKeyToPemError)
    }
}

pub fn from_pem(str: &str) -> Result<RsaPrivateKey, RSAError> {
    let result = RsaPrivateKey::from_pkcs8_pem(str);
    if result.is_ok() {
        Ok(result.unwrap())
    } else {
        Err(RSAError::PemToRsaKeyError)
    }
}