use chrono::Utc;
use sea_orm::{DbErr, EntityTrait};
use entity::prelude::SessionStorage;
use entity::session_storage;
use crate::db::connection::db;

#[derive(Debug)]
pub enum SessionError {
    SessionNotFound,
}

pub async fn find_session(session_id: i64) -> Result<session_storage::Model, SessionError> {
    let db = db().await.unwrap();
    let result: Result<Option<session_storage::Model>, DbErr> = SessionStorage::find_by_id(session_id).one(&db).await;
    if result.is_ok() {
        let result = result.unwrap();
        if result.is_none() {
            Err(SessionError::SessionNotFound)
        } else {
            Ok(result.unwrap())
        }
    } else {
        Err(SessionError::SessionNotFound)
    }
}

pub async fn expired(session_id: i64) -> Result<bool, SessionError> {
    let result = find_session(session_id).await;
    if result.is_ok() {
        let result = result?;
        if result.expire < Utc::now().naive_local() {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Err(result.unwrap_err())
    }
}