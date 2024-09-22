use entity::prelude::Blog;
use entity::blog;
use sea_orm::{Condition, DatabaseConnection, EntityTrait, QuerySelect};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use warp::{Rejection, Reply};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogRequest {
    owner_id: Option<String>, // Blog's owner ID, None for request for all blogs
    offset: Option<i32>, // Offset of the blogs, default is 0
    limit: Option<i32>, // Return blogs for each time, default is 10
    is_draft: Option<bool>, // Request for draft, default is false
}

pub async fn get_blogs_handler(blog_request: BlogRequest, db: DatabaseConnection) -> Result<Box<dyn Reply>, Rejection> {
    let blog_request = handle_blog_request(blog_request);
    let owner_id = blog_request.get("owner_id").unwrap().as_str().unwrap().to_string();
    let offset = blog_request.get("offset").unwrap().as_i64().unwrap() as i32;
    let limit = blog_request.get("limit").unwrap().as_i64().unwrap() as i32;
    let is_draft = blog_request.get("is_draft").unwrap().as_bool().unwrap();

    let result = Blog::find()
        .offset(offset)
        .limit(limit)
        .filter(
            Condition::all()
                .add(blog::Column::UserId.eq(owner_id))
                .add(blog::Column::IsDraft.eq(is_draft))
        )
        .all(&db)
        .await;
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