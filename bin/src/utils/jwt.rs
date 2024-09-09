use crate::db::connection::db;
use chrono;
use chrono::{Duration, Utc};
use entity::prelude::{JwtSecret, User};
use entity::{jwt_secret, user};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rand::Rng;
use sea_orm::{DbErr, EntityTrait};
use serde::{Deserialize, Serialize};
use crate::utils::error::{Error, JWTError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    username: String,
    password: String,
    exp: usize, // Expiration time (as UTC timestamp)
}

async fn generate_claims(user_id: i64) -> Result<Option<Claims>, DbErr> {
    let db = db().await.unwrap();
    let result: Result<Option<user::Model>, DbErr> = User::find_by_id(user_id).one(&db).await;
    if result.is_ok() {
        let user = result?;
        let claims: Option<Claims> = if user.is_none() {
            None
        } else {
            let user = user.unwrap();
            let some_claims = Claims {
                username: user.user_name,
                password: user.user_password,
                exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            };
            Some(some_claims)
        };
        Ok(claims)
    } else {
        Err(result.unwrap_err())
    }
}

async fn get_secret(user_id: i64) -> Result<Option<String>, DbErr> {
    let db = db().await.unwrap();
    let result: Result<Option<jwt_secret::Model>, DbErr> = JwtSecret::find_by_id(user_id).one(&db).await;
    if result.is_ok() {
        let jwt_secret = result?;
        let secret = if jwt_secret.is_none() {
            None
        } else {
            let jwt_secret = jwt_secret.unwrap();
            Some(jwt_secret.user_secret)
        };
        Ok(secret)
    } else {
        Err(result.unwrap_err())
    }
}

pub fn generate_secret() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
    const SECRET_LEN: usize = 30;
    let mut rng = rand::thread_rng();
    let secret: String = (0..SECRET_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    secret
}

pub async fn generate_jwt(user_id: i64) -> Result<String, Error> {
    let claims = generate_claims(user_id).await;
    if claims.is_ok() {
        let claims = claims.unwrap();
        if claims.is_none() {
            Err(JWTError::UserDoesNotExist.error())
        } else {
            let header = Header::new(Algorithm::HS512);
            let secret = get_secret(user_id).await;
            if secret.is_ok() {
                let secret = secret.unwrap();
                if secret.is_none() {
                    Err(JWTError::SecretDoesNotExist.error())
                } else {
                    let encoding_key = EncodingKey::from_secret(secret.unwrap().as_ref());
                    let token = encode(&header, &claims, &encoding_key).unwrap();
                    Ok(token)
                }
            } else {
                Err(JWTError::SomeErrorsOccurred.error())
            }
        }
    } else {
        Err(JWTError::SomeErrorsOccurred.error())
    }
}

pub async fn validate_claims(user_id: i64, token_data: TokenData<Claims>) -> bool {
    let db = db().await.unwrap();
    let result: Result<Option<user::Model>, DbErr> = User::find_by_id(user_id).one(&db).await;
    if result.is_ok() {
        let user = result.unwrap();
        if user.is_none() {
            false
        } else {
            let user = user.unwrap();
            let username = user.user_name;
            let password = user.user_password;
            if username == token_data.claims.username && password == token_data.claims.password {
                if Utc::now().timestamp() as usize > token_data.claims.exp {
                    false
                } else {
                    true
                }
            } else {
                false
            }
        }
    } else {
        false
    }
}

pub async fn decode_jwt(user_id: i64, token: &str) -> Result<TokenData<Claims>, Error> {
    let validation = Validation::new(Algorithm::HS512);
    let secret = get_secret(user_id).await;
    if secret.is_ok() {
        let secret = secret.unwrap();
        if secret.is_none() {
            Err(JWTError::SecretDoesNotExist.error())
        } else {
            let decoding_key = DecodingKey::from_secret(secret.unwrap().as_ref());
            let token = decode::<Claims>(&token, &decoding_key, &validation).unwrap();
            Ok(token)
        }
    } else {
        Err(JWTError::SomeErrorsOccurred.error())
    }
}