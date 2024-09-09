mod filter;
mod handler;

use warp::Filter;
use db::connection::db;
use migration::cli;
use crate::filter::with_db::with_db;
use crate::handler::*;
use crate::handler::login_handler::UserRequest;

#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
    let db = db().await.unwrap();

    let rsa_key = warp::path!("api" / "key")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(rsa_handler::rsa_handler);

    let login = warp::path!("api" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .map(|request: UserRequest| {
            request
        })
        .and(warp::cookie::<String>("sid"))
        .and(with_db(db.clone()))
        .and_then(login_handler::login_handler);

    let token_login = warp::path!("api" / "login" / "token")
        .and(warp::post())
        .and(warp::cookie::<String>("token"))
        .and(warp::cookie::<String>("sid"))
        .and(with_db(db.clone()))
        .and_then(token_login_handler::token_login_handler);

    let routes = rsa_key
        .or(login)
        .or(token_login);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
