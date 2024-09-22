use crate::server_utils::utils::{token_request_ok, token_request_fail, verify_session, verify_token};
use entity::blog;
use entity::prelude::Blog;
use sea_orm::{Condition, DatabaseConnection, DbErr, EntityTrait, QuerySelect, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use utils::error::{ApiResultError, ZenithError};
use utils::jwt::generate_jwt;
use utils::session::extract_user_id;
use warp::http::StatusCode;
use warp::{Rejection, Reply};
use utils::result::success_with_data;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogRequest {
    owner_id: Option<String>, // Blog's owner ID, None for request for all blogs
    offset: Option<i32>, // Offset of the blogs, default is 0
    limit: Option<i32>, // Return blogs for each time, default is 10
    is_draft: Option<bool>, // Request for draft, default is false
}

pub async fn get_blogs_handler(blog_request: BlogRequest, db: DatabaseConnection) -> Result<Box<dyn Reply>, Rejection> {
    let list = result_list(blog_request, db.clone()).await;
    if list.is_ok() {
        let list = list.unwrap();
        if list.is_none() {
            let vec: Vec<Value> = vec![];
            Ok(Box::new(warp::reply::json(&success_with_data(json!({
                "list": vec
            })))))
        } else {
            let list = list.unwrap();
            Ok(Box::new(warp::reply::json(&success_with_data(json!({
                "list": list
            })))))
        }
    } else {
        let vec: Vec<Value> = vec![];
        Ok(Box::new(warp::reply::json(&success_with_data(json!({
            "list": vec
        })))))
    }
}

pub async fn get_drafts_handler(blog_request: BlogRequest, token: String, session_id: String, db: DatabaseConnection) -> Result<Box<dyn Reply>, Rejection> {
    let session_result = verify_session(session_id.parse::<i64>().unwrap(), db.clone()).await;
    if session_result {
        let user_id = extract_user_id(session_id.parse::<i64>().unwrap(), db.clone()).await.unwrap();
        let verify_result = verify_token(user_id, token).await;

        if verify_result {
            let jwt_result = generate_jwt(user_id).await;
            if jwt_result.is_ok() {
                let jwt_result = jwt_result.unwrap();
                let list = result_list(blog_request, db.clone()).await;
                if list.is_ok() {
                    let list = list.unwrap();
                    if list.is_none() {
                        let vec: Vec<Value> = vec![];
                        Ok(token_request_ok(user_id, session_id.parse::<i64>().unwrap(), jwt_result, json!({
                            "list": vec
                        }), db).await)
                    } else {
                        let list = list.unwrap();
                        Ok(token_request_ok(user_id, session_id.parse::<i64>().unwrap(), jwt_result, json!({
                            "list": list
                        }), db).await)
                    }
                } else {
                    let vec: Vec<Value> = vec![];
                    Ok(token_request_ok(user_id, session_id.parse::<i64>().unwrap(), jwt_result, json!({
                        "list": vec
                    }), db).await)
                }
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

fn handle_blog_request(blog_request: BlogRequest) -> Value {
    // get owner_id from json
    let owner_id = if blog_request.owner_id.is_none() {
        0.to_string()
    } else { blog_request.owner_id.unwrap() };

    // get offset from json
    let offset = if blog_request.offset.is_none() {
        0
    } else { blog_request.offset.unwrap() };

    // get limit from json
    let limit = if blog_request.limit.is_none() {
        10
    } else { blog_request.limit.unwrap() };

    // get is_draft from json
    let is_draft = if blog_request.is_draft.is_none() {
        false
    } else { blog_request.is_draft.unwrap() };

    json!({
        "owner_id": owner_id,
        "offset": offset,
        "limit": limit,
        "is_draft": is_draft
    })
}

async fn result_list(blog_request: BlogRequest, db: DatabaseConnection) -> Result<Option<Vec<Value>>, DbErr> {
    let blog_request = handle_blog_request(blog_request);
    let owner_id = blog_request.get("owner_id").unwrap().as_str().unwrap().to_string();
    let offset = blog_request.get("offset").unwrap().as_u64().unwrap();
    let limit = blog_request.get("limit").unwrap().as_u64().unwrap();
    let is_draft = blog_request.get("is_draft").unwrap().as_bool().unwrap();

    async fn find_result(owner_id:String, offset: u64, limit:u64, is_draft: bool, db: DatabaseConnection) -> Result<Option<Vec<Value>>, DbErr> {
        let result: Result<Vec<blog::Model>, DbErr> = Blog::find()
            .offset(offset)
            .limit(limit)
            .filter(
                Condition::all()
                    .add(blog::Column::UserId.eq(owner_id))
                    .add(blog::Column::IsDraft.eq(is_draft))
            )
            .all(&db).await;

        if result.is_ok() {
            let result = result?;
            if result.is_empty() {
                Ok(None)
            } else {
                let mut vec: Vec<Value> = Vec::new();
                for r in &result {
                    vec.push(json!({
                    "blog_id": r.blog_id,
                    "user_id": r.user_id,
                    "category_id": r.category_id,
                    "blog_description": r.blog_description,
                    "blog_pub_dt": r.blog_pub_dt
                }));
                }
                Ok(Some(vec))
            }
        } else {
            Err(result.unwrap_err())
        }
    }

    match is_draft {
        true => {
            if owner_id == 0.to_string() {
                Ok(None)
            } else {
                find_result(owner_id, offset, limit, is_draft, db.clone()).await
            }
        }
        false => {
            find_result(owner_id, offset, limit, is_draft, db.clone()).await
        }
    }
}