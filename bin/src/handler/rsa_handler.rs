use chrono::{Duration, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use warp::{Rejection, Reply};
use warp::http::StatusCode;
use entity::session_storage;
use crate::utils::id::generate_id;
use crate::utils::rsa_utils::{generate_key_pair, pub_to_pem, to_pem};

pub async fn rsa_handler(db: DatabaseConnection) -> Result<Box<dyn Reply>, Rejection> {
    let id = generate_id();
    let key_pair = generate_key_pair();
    let pub_pem = pub_to_pem(key_pair.public_key);
    let pem = to_pem(key_pair.private_key);

    if pem.is_ok() {
        let session = session_storage::ActiveModel {
            session_id: Set(id.to_owned()),
            rsa_pem: Set(Some(pem.unwrap()).to_owned()),
            user_id: Default::default(),
            expire: Set((Utc::now() + Duration::hours(1)).naive_local().to_owned()),
        };
        let result = session.insert(&db).await;
        if result.is_ok() {
            if pub_pem.is_ok() {
                Ok(Box::new(
                    warp::reply::with_header(
                        pub_pem.unwrap(),
                        "set-cookie",
                        format!("sid={}; Path=/; HttpOnly; Max-Age={}", id.to_string(), (60 * 60 * 24 * 7)),
                    ).into_response()
                ))
            } else {
                Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
            }
        } else {
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    } else {
        Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
    }
}