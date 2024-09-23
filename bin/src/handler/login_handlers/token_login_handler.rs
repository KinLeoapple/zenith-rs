use crate::common::utils::{token_request_fail, token_request_ok, verify_session, verify_token};
use sea_orm::DatabaseConnection;
use serde_json::json;
use utils::error::{ApiResultError, ZenithError};
use utils::jwt::generate_jwt;
use utils::session::extract_user_id;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn token_login_handler(token: String, session_id: String, db: DatabaseConnection) -> Result<Box<dyn Reply>, Rejection> {
    let session_result = verify_session(session_id.parse::<i64>().unwrap(), db.clone()).await;
    if session_result {
        let user_id = extract_user_id(session_id.parse::<i64>().unwrap(), db.clone()).await.unwrap();
        let verify_result = verify_token(user_id, token).await;

        if verify_result {
            let jwt_result = generate_jwt(user_id).await;
            if jwt_result.is_ok() {
                let jwt_result = jwt_result.unwrap();
                Ok(token_request_ok(user_id, session_id.parse::<i64>().unwrap(), jwt_result, json!({
                            "id": user_id,
                        }), db).await)
            } else {
                Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
            }
        } else {
            let result_error = ApiResultError::InvalidToken.error();
            Ok(token_request_fail(result_error.code, &result_error.message))
        }
    } else {
        Ok(Box::new(StatusCode::UNAUTHORIZED))
    }
}