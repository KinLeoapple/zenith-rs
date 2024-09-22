use sea_orm::DatabaseConnection;
use warp::{Filter, Rejection, Reply};
use crate::filter::with_db::with_db;
use crate::handler::blog_handlers::get_blogs_handler::{get_drafts_handler, BlogRequest};

pub fn get_drafts(db: DatabaseConnection) -> impl Filter<Extract=(Box<(dyn Reply + 'static)>,), Error=Rejection> + Clone + Send + Sync + 'static {
    // get drafts
    let get_drafts = warp::path!("api" / "drafts")
        .and(warp::get())
        .and(warp::body::json())
        .map(|request: BlogRequest| {
            request
        })
        .and(warp::cookie::<String>("token"))
        .and(warp::cookie::<String>("sid"))
        .and(with_db(db.clone()))
        .and_then(get_drafts_handler);
    get_drafts
}