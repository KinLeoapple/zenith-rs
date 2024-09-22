use sea_orm::DatabaseConnection;
use warp::{Filter, Rejection, Reply};
use crate::filter::with_db::with_db;
use crate::handler::rsa_handler;

pub fn rsa_key(db: DatabaseConnection) -> impl Filter<Extract=(Box<(dyn Reply + 'static)>,), Error=Rejection> + Clone + Send + Sync + 'static {
    // RSA key
    let rsa_key = warp::path!("api" / "key")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(rsa_handler::rsa_handler);
    rsa_key
}