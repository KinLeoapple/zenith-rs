use crate::common::utils::{token_request_fail, token_request_ok};
use entity::prelude::User;
use entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utils::error::{ApiResultError, ZenithError};
use utils::hex::HexString;
use utils::jwt::generate_jwt;
use utils::result::failure;
use utils::rsa::{decrypt, from_pem};
use utils::session::find_session;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    username: String, // The name of the user
    password: String, // The password of the user
}

pub async fn login_handler(user_request: UserRequest, session_id: String, db: DatabaseConnection) -> Result<Box<dyn Reply>, Rejection> {
    let result = User::find()
        .filter(user::Column::UserName.eq(&user_request.username))
        .all(&db)
        .await;

    if result.is_ok() {
        let result = result.unwrap();
        if result.is_empty() {
            let result_error = ApiResultError::UserDoesNotExist.error();
            let api_result = failure(result_error.code, &result_error.message);
            Ok(Box::new(warp::reply::json(&api_result)))
        } else {
            let password = decode_password(&user_request.password, session_id.parse::<i64>().unwrap(), db.clone()).await;
            if password.is_none() {
                Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
            } else {
                let password = password.unwrap();
                if result[0].user_name == user_request.username
                    && result[0].user_password == password {
                    let jwt_result = generate_jwt(result[0].user_id).await;
                    println!("{:?}", jwt_result);
                    if jwt_result.is_ok() {
                        let jwt_result = jwt_result.unwrap();
                        Ok(token_request_ok(result[0].user_id, session_id.parse::<i64>().unwrap(), jwt_result, json!({
                            "id": result[0].user_id,
                        }), db).await)
                    } else {
                        Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
                    }
                } else {
                    let result_error = ApiResultError::UserPasswordDoesNotMatch.error();
                    Ok(token_request_fail(result_error.code, &result_error.message))
                }
            }
        }
    } else {
        Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
    }
}

async fn decode_password(encrypted_password: &str, session_id: i64, db: DatabaseConnection) -> Option<String> {
    let result = find_session(session_id, db).await;
    if result.is_ok() {
        let result = result.unwrap();
        if result.rsa_pem.is_none() {
            None
        } else {
            let priv_key = from_pem(&result.rsa_pem?).unwrap();
            let decrypted_password = decrypt(HexString {
                hex: encrypted_password.to_string(),
            }, priv_key);
            if decrypted_password.is_ok() {
                let password = decrypted_password.unwrap();
                Some(password.hex)
            } else {
                None
            }
        }
    } else {
        None
    }
}