use sea_orm::DatabaseConnection;
use warp::{Filter, Rejection, Reply};
use crate::filter::with_db::with_db;
use crate::handler::login_handlers::token_login_handler::token_login_handler;

pub fn token_login(db: DatabaseConnection) -> impl Filter<Extract=(Box<(dyn Reply + 'static)>,), Error=Rejection> + Clone + Send + Sync + 'static {
    // login with token
    let token_login = warp::path!("api" / "login" / "token")
        .and(warp::post())
        .and(warp::cookie::<String>("token"))
        .and(warp::cookie::<String>("sid"))
        .and(with_db(db.clone()))
        .and_then(token_login_handler);
    token_login
}