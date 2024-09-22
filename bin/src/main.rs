mod filter;
mod handler;
mod routes;
mod server_utils;

use crate::routes::get_blogs::get_blogs;
use crate::routes::get_drafts::get_drafts;
use crate::routes::login::login;
use crate::routes::rsa_key::rsa_key;
use crate::routes::token_login::token_login;
use anyhow::Result;
use db::connection::db;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use utils::banner::display_banner;
use utils::server_info::display_server_info;
use utils::media;
use warp::Filter;

const PORT: u16 = 3030;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}

async fn run() -> Result<()> {
    display_banner();
    media::create_folder();
    // migration
    let db = db().await.unwrap();
    db_migration(db.clone()).await?;
    // run server
    display_server_info(format!("{} {}{}", "Server is running on port", PORT, "..."));
    run_server(db.clone()).await;
    Ok(())
}

async fn run_server(db: DatabaseConnection) {
    let routes = rsa_key(db.clone())
        .or(login(db.clone()))
        .or(token_login(db.clone()))
        .or(get_blogs(db.clone()))
        .or(get_drafts(db.clone()));
    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
}

async fn db_migration(db: DatabaseConnection) -> Result<()> {
    Migrator::up(&db, None).await?;
    Ok(())
}
