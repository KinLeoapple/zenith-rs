use sea_orm::DatabaseConnection;
use serde_json::Value;
use warp::Reply;
use utils::jwt::{decode_jwt, verify_claims};
use utils::result::{failure, success_with_data};
use utils::session::{extend_time, find_session, session_owned};

pub async fn token_request_ok(user_id: i64, session_id: i64, jwt: String, json: Value, db: DatabaseConnection) -> Box<dyn Reply> {
    let api_result = success_with_data(json);
    session_owned(session_id, user_id, db.clone()).await;
    extend_time(session_id, db).await;

    Box::new(
        warp::reply::with_header(
            serde_json::to_string(&api_result).unwrap(),
            "set-cookie",
            format!("token={}; Path=/; HttpOnly; Max-Age={}", jwt, 60 * 60 * 24 * 7),
        ).into_response()
    )
}

pub fn token_request_fail(code: i32, message: &str) -> Box<dyn Reply> {
    let api_result = failure(code, message);
    Box::new(warp::reply::json(&api_result))
}

pub async fn verify_session(session_id: i64, db: DatabaseConnection) -> bool {
    let result = find_session(session_id, db.clone()).await;
    if result.is_ok() {
        let result = result.unwrap();
        if result.user_id.is_none() {
            false
        } else {
            true
        }
    } else {
        false
    }
}

pub async fn verify_token(user_id: i64, token: String) -> bool {
    let decoded = decode_jwt(user_id, &token).await;
    if decoded.is_ok() {
        let decoded = decoded.unwrap();
        let verify_result = verify_claims(decoded).await;
        verify_result
    } else {
        false
    }
}