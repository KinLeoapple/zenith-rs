use std::env;
use std::time::Duration;
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};


pub async fn db() -> Option<DatabaseConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));
    let connection = Database::connect(opt).await.unwrap();
    
    let ping = connection.ping().await;
    if ping.is_ok() {
        Some(connection)
    } else {
        None
    }
}