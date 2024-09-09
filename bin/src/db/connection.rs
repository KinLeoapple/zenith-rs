use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

const USERNAME: &str = "postgres";
const PASSWORD: &str = "123456";
const DATABASE: &str = "zenith";

pub async fn db() -> Option<DatabaseConnection> {
    let database_url = format!("postgres://{}:{}@localhost:5432/{}", USERNAME, PASSWORD, DATABASE);
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));
    let connection = Database::connect(opt).await.unwrap();
    
    let ping = connection.ping().await;
    if (ping.is_ok()) {
        Some(connection)
    } else {
        None
    }
}