use sea_orm::DatabaseConnection;
use warp::{Filter, Rejection, Reply};
use crate::filter::with_db::with_db;
use crate::handler::login_handlers::login_handler::{login_handler, UserRequest};

pub fn login(db: DatabaseConnection) -> impl Filter<Extract=(Box<(dyn Reply + 'static)>,), Error=Rejection> + Clone + Send + Sync + 'static {
    // login
    let login = warp::path!("api" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .map(|request: UserRequest| {
            request
        })
        .and(warp::cookie::<String>("sid"))
        .and(with_db(db.clone()))
        .and_then(login_handler);
    login
}