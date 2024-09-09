use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::{Rejection, Reply};
use warp::http::StatusCode;
use entity::prelude::User;
use entity::user;
use crate::utils::error::ApiResultError;
use crate::utils::jwt::generate_jwt;
use crate::utils::result::{failure, success_with_data};
use crate::utils::session::session_owned;

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
            let api_result = failure(result_error.code, &result_error.message.message);
            Ok(Box::new(warp::reply::json(&api_result)))
        } else {
            if result[0].user_name == user_request.username
                && result[0].user_password == user_request.password {
                let jwt_result = generate_jwt(result[0].user_id).await;
                if jwt_result.is_ok() {
                    let jwt_result = jwt_result.unwrap();
                    let api_result = success_with_data(json!({
                        "id": result[0].user_id
                    }));
                    session_owned(session_id.parse::<i64>().unwrap(), result[0].user_id).await;

                    Ok(Box::new(
                        warp::reply::with_header(
                            serde_json::to_string(&api_result).unwrap(),
                            "set-cookie",
                            format!("token={}; Path=/; HttpOnly; Max-Age={}", jwt_result, (60 * 60 * 24 * 7)),
                        ).into_response()
                    ))
                } else {
                    Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
                }
            } else {
                let result_error = ApiResultError::UserPasswordDoesNotMatch.error();
                let api_result = failure(result_error.code, &result_error.message.message);
                Ok(Box::new(warp::reply::json(&api_result)))
            }
        }
    } else {
        Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
    }
}