use sea_orm::{Database, DatabaseConnection};

const USERNAME: &str = "postgres";
const PASSWORD: &str = "123456";
const DATABASE: &str = "zenith";

pub async fn db() -> Option<DatabaseConnection> {
    let database_url = format!("postgres://{}:{}@localhost:5432/{}", USERNAME, PASSWORD, DATABASE);
    let connection = Database::connect(&database_url).await.unwrap();
    let ping = connection.ping().await;
    if (ping.is_ok()) {
        Some(connection)
    } else {
        None
    }
}