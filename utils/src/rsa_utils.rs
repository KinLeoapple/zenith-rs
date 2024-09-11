use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPublicKey, LineEnding};
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use crate::error::{Error, RSAError, ZenithError};
use crate::hex::{to_hex, to_vec, HexString};

#[derive(Debug)]
pub struct RsaKeyPair {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
}

pub fn generate_key_pair() -> RsaKeyPair {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let pub_key = RsaPublicKey::from(&priv_key);
    RsaKeyPair {
        private_key: priv_key.to_owned(),
        public_key: pub_key.to_owned(),
    }
}

pub fn encrypt(str: &str, pub_key: RsaPublicKey) -> Result<HexString, Error> {
    let data = str.as_bytes();
    let mut rng = rand::thread_rng();
    let result = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]);
    if result.is_ok() {
        Ok(to_hex(result.unwrap().as_slice()))
    } else {
        Err(RSAError::RsaEncryptionError.error())
    }
}

pub fn decrypt(hex_str: HexString, priv_key: RsaPrivateKey) -> Result<HexString, Error> {
    let data = to_vec(&hex_str.hex);
    if data.is_ok() {
        let data = data.unwrap();
        let result = priv_key.decrypt(Pkcs1v15Encrypt, &data);
        if result.is_ok() {
            Ok(to_hex(result.unwrap().as_slice()))
        } else {
            Err(RSAError::RsaDecryptionError.error())
        }
    } else {
        Err(RSAError::RsaDecryptionError.error())
    }
}

pub fn pub_to_pem(pub_key: RsaPublicKey) -> Result<String, Error> {
    let result = pub_key.to_pkcs1_pem(LineEnding::CR);
    if result.is_ok() {
        Ok(result.unwrap().to_string())
    } else {
        Err(RSAError::RsaKeyToPemError.error())
    }
}

pub fn to_pem(priv_key: RsaPrivateKey) -> Result<String, Error> {
    let result = priv_key.to_pkcs8_pem(LineEnding::CR);
    if result.is_ok() {
        Ok(result.unwrap().to_string())
    } else {
        Err(RSAError::RsaKeyToPemError.error())
    }
}

pub fn from_pem(str: &str) -> Result<RsaPrivateKey, Error> {
    let result = RsaPrivateKey::from_pkcs8_pem(str);
    if result.is_ok() {
        Ok(result.unwrap())
    } else {
        Err(RSAError::PemToRsaKeyError.error())
    }
}