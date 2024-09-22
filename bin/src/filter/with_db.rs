use sea_orm::DatabaseConnection;
use std::convert::Infallible;
use warp::Filter;

pub fn with_db(db: DatabaseConnection) -> impl Filter<Extract=(DatabaseConnection,), Error=Infallible> + Clone {
    warp::any().map(move || db.clone())
}