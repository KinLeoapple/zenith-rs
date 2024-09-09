use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: i32,
    #[serde(flatten)]
    pub message: Error,
}

#[derive(Debug)]
pub enum RSAError {
    RsaEncryptionError,
    RsaDecryptionError,
    RsaKeyToPemError,
    PemToRsaKeyError,
}

impl RSAError {
    pub fn error(&self) -> Error {
        match self {
            Self::RsaEncryptionError => Error { message: "RSA Encryption Error".to_string() },
            Self::RsaDecryptionError => Error { message: "RSA Decryption Error".to_string() },
            Self::RsaKeyToPemError => Error { message: "RSA Key To Pem Error".to_string() },
            Self::PemToRsaKeyError => Error { message: "Pem To RSA Key Error".to_string() },
        }
    }
}

#[derive(Debug)]
pub enum SessionError {
    SessionNotFound,
    SessionExpired,
}

impl SessionError {
    pub fn error(&self) -> Error {
        match self {
            Self::SessionNotFound => Error { message: "Session Not Found".to_string() },
            Self::SessionExpired => Error { message: "Session Expired".to_string() },
        }
    }
}

#[derive(Debug)]
pub enum JWTError {
    UserDoesNotExist,
    SecretDoesNotExist,
    SomeErrorsOccurred,
}

impl JWTError {
    pub fn error(&self) -> Error {
        match self {
            Self::UserDoesNotExist => Error { message: "User Does Not Exist".to_string() },
            Self::SecretDoesNotExist => Error { message: "Secret Does Not Exist".to_string() },
            Self::SomeErrorsOccurred => Error { message: "Some Errors".to_string() },
        }
    }
}

#[derive(Debug)]
pub enum ApiResultError {
    UserDoesNotExist,
    UserPasswordDoesNotMatch,
    InvalidToken,
}

impl ApiResultError {
    pub fn error(&self) -> ApiError {
        match self {
            Self::UserDoesNotExist =>
                ApiError { code: 2001, message: Error { message: "User Does Not Exist".to_string() } },
            Self::UserPasswordDoesNotMatch =>
                ApiError { code: 2002, message: Error { message: "User Password Does Not Match".to_string() } },
            Self::InvalidToken =>
                ApiError { code: 2003, message: Error { message: "Invalid Token".to_string() } },
        }
    }
}