use sea_orm::{Database, DatabaseConnection, DbErr};

const USERNAME: &str = "postgres";
const PASSWORD: &str = "123456";
const DATABASE: &str = "zenith";

pub async fn db_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = format!("postgres://{}:{}@localhost:5432/{}", USERNAME, PASSWORD, DATABASE);
    let connection = Database::connect(&database_url).await;
    connection
}