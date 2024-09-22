mod filter;
mod handler;
mod routes;

use crate::routes::login::login;
use crate::routes::rsa_key::rsa_key;
use crate::routes::token_login::token_login;
use anyhow::Result;
use db::connection::db;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use utils::banner::banner;
use utils::media;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}

async fn run() -> Result<()> {
    println!("{}", banner());
    media::create_folder();
    // migration
    let db = db().await.unwrap();
    db_migration(db.clone()).await?;
    // run server
    run_server(db.clone()).await;
    Ok(())
}

async fn run_server(db: DatabaseConnection) {
    let routes = rsa_key(db.clone())
        .or(login(db.clone()))
        .or(token_login(db.clone()));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn db_migration(db: DatabaseConnection) -> Result<()> {
    Migrator::refresh(&db).await?;
    Ok(())
}
