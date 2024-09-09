mod db;
mod utils;
mod filter;
mod handler;

use warp::Filter;
use crate::db::connection::db;
use crate::filter::with_db::with_db;
use crate::handler::*;

#[tokio::main]
async fn main() {
    let db = db().await.unwrap();

    let rsa_key = warp::path!("api" / "key")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(rsa_handler::rsa_handler);

    let routes = rsa_key;
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
