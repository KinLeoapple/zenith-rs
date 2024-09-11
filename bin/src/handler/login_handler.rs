use entity::prelude::User;
use entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utils::error::{ApiResultError, ZenithError};
use utils::jwt::generate_jwt;
use utils::result::{failure, success_with_data};
use utils::session::{extend_time, find_session, session_owned};
use warp::http::StatusCode;
use warp::{Rejection, Reply};
use utils::hex::to_hex;
use utils::rsa::{decrypt, from_pem};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    username: String,
    password: String,
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
                    if jwt_result.is_ok() {
                        let jwt_result = jwt_result.unwrap();
                        Ok(login_ok(result[0].user_id, session_id.parse::<i64>().unwrap(), jwt_result, db).await)
                    } else {
                        Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
                    }
                } else {
                    let result_error = ApiResultError::UserPasswordDoesNotMatch.error();
                    Ok(login_fail(result_error.code, &result_error.message))
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
            let decrypted_password = decrypt(to_hex(encrypted_password.as_bytes()), priv_key);
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

pub async fn login_ok(user_id: i64, session_id: i64, jwt: String, db: DatabaseConnection) -> Box<dyn Reply> {
    let api_result = success_with_data(json!({
                        "id": user_id,
                    }));
    session_owned(session_id, user_id, db.clone()).await;
    extend_time(session_id, db).await;

    Box::new(
        warp::reply::with_header(
            serde_json::to_string(&api_result).unwrap(),
            "set-cookie",
            format!("token={}; Path=/; HttpOnly; Max-Age={}", jwt, (60 * 60 * 24 * 7)),
        ).into_response()
    )
}

pub fn login_fail(code: i32, message: &str) -> Box<dyn Reply> {
    let api_result = failure(code, message);
    Box::new(warp::reply::json(&api_result))
}