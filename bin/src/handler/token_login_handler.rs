use sea_orm::DatabaseConnection;
use warp::{Rejection, Reply};

pub async fn token_login_handler(token: String, db: DatabaseConnection) -> Result<Box<dyn Reply>, Rejection> {
    todo!()
}