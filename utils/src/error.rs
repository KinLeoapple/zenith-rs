use serde::{Deserialize, Serialize};

pub trait ZenithError {
    fn error(&self) -> Error;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Debug)]
pub enum RSAError {
    RsaEncryptionError,
    RsaDecryptionError,
    RsaKeyToPemError,
    PemToRsaKeyError,
}

impl ZenithError for RSAError {
    fn error(&self) -> Error {
        match self {
            Self::RsaEncryptionError => Error { code: -1, message: "RSA Encryption Error".to_string() },
            Self::RsaDecryptionError => Error { code: -1, message: "RSA Decryption Error".to_string() },
            Self::RsaKeyToPemError => Error { code: -1, message: "RSA Key To Pem Error".to_string() },
            Self::PemToRsaKeyError => Error { code: -1, message: "Pem To RSA Key Error".to_string() },
        }
    }
}

#[derive(Debug)]
pub enum SessionError {
    SessionNotFound,
    SessionExpired,
}

impl ZenithError for SessionError {
    fn error(&self) -> Error {
        match self {
            Self::SessionNotFound => Error { code: -1, message: "Session Not Found".to_string() },
            Self::SessionExpired => Error { code: -1, message: "Session Expired".to_string() },
        }
    }
}

#[derive(Debug)]
pub enum JWTError {
    UserDoesNotExist,
    SecretDoesNotExist,
    SomeErrorsOccurred,
}

impl ZenithError for JWTError {
    fn error(&self) -> Error {
        match self {
            Self::UserDoesNotExist => Error { code: -1, message: "User Does Not Exist".to_string() },
            Self::SecretDoesNotExist => Error { code: -1, message: "Secret Does Not Exist".to_string() },
            Self::SomeErrorsOccurred => Error { code: -1, message: "Some Errors".to_string() },
        }
    }
}

#[derive(Debug)]
pub enum ApiResultError {
    UserDoesNotExist,
    UserPasswordDoesNotMatch,
    InvalidToken,
}

impl ZenithError for ApiResultError {
    fn error(&self) -> Error {
        match self {
            Self::UserDoesNotExist =>
                Error { code: 2001, message: "User Does Not Exist".to_string() },
            Self::UserPasswordDoesNotMatch =>
                Error { code: 2002, message: "User Password Does Not Match".to_string() },
            Self::InvalidToken =>
                Error { code: 2003, message: "Invalid Token".to_string() },
        }
    }
}