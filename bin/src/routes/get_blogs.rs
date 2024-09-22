use sea_orm::DatabaseConnection;
use warp::{Filter, Rejection, Reply};

use crate::filter::with_db::with_db;
use crate::handler::blog_handlers::get_blogs_handler::{get_blogs_handler, BlogRequest};

pub fn get_blogs(db: DatabaseConnection) -> impl Filter<Extract=(Box<(dyn Reply + 'static)>,), Error=Rejection> + Clone + Send + Sync + 'static {
    // get blogs
    let get_blogs = warp::path!("api" / "blogs")
        .and(warp::get())
        .and(warp::body::json())
        .map(|request: BlogRequest| {
            request
        })
        .and(with_db(db.clone()))
        .and_then(get_blogs_handler);
    get_blogs
}