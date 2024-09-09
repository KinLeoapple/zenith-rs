use crate::db::connection::db;
use crate::utils::error::{Error, SessionError};
use chrono::Utc;
use entity::prelude::SessionStorage;
use entity::session_storage;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};

pub async fn find_session(session_id: i64) -> Result<session_storage::Model, Error> {
    let db = db().await.unwrap();
    let result: Result<Option<session_storage::Model>, DbErr> = SessionStorage::find_by_id(session_id).one(&db).await;
    if result.is_ok() {
        let result = result.unwrap();
        if result.is_none() {
            Err(SessionError::SessionNotFound.error())
        } else {
            Ok(result.unwrap())
        }
    } else {
        Err(SessionError::SessionNotFound.error())
    }
}

pub async fn expired(session_id: i64) -> Result<bool, Error> {
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

pub async fn session_owned(session_id: i64, user_id: i64) {
    let result = find_session(session_id).await;
    if result.is_ok() {
        let db = db().await.unwrap();
        let mut result: session_storage::ActiveModel = result.unwrap().into();
        result.user_id = Set(Some(user_id.to_owned()));
        result.update(&db).await.unwrap();
    }
}