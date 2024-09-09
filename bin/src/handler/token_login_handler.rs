use sea_orm::DatabaseConnection;
use warp::{Rejection, Reply};
use warp::http::StatusCode;
use utils::error::ApiResultError;
use utils::jwt::{decode_jwt, generate_jwt, verify_claims};
use utils::session::find_session;
use crate::handler::login_handler::{login_fail, login_ok};

pub async fn token_login_handler(token: String, session_id: String, db: DatabaseConnection) -> Result<Box<dyn Reply>, Rejection> {
    let result = find_session(session_id.parse::<i64>().unwrap(), db.clone()).await;
    if result.is_ok() {
        let result = result.unwrap();
        if result.user_id.is_none() {
            Ok(Box::new(StatusCode::UNAUTHORIZED))
        } else {
            let user_id = result.user_id.unwrap();
            let decoded = decode_jwt(user_id, &token).await;

            if decoded.is_ok() {
                let decoded = decoded.unwrap();
                let verify_result = verify_claims(decoded).await;

                if verify_result {
                    let jwt_result = generate_jwt(user_id).await;
                    if jwt_result.is_ok() {
                        let jwt_result = jwt_result.unwrap();
                        Ok(login_ok(user_id, session_id.parse::<i64>().unwrap(), jwt_result, db).await)
                    } else {
                        Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
                    }
                } else {
                    let result_error = ApiResultError::InvalidToken.error();
                    Ok(login_fail(result_error.code, &result_error.message.message))
                }
            } else {
                Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
    } else {
        Ok(Box::new(StatusCode::UNAUTHORIZED))
    }
}