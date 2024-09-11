use chrono::{Duration, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use entity::prelude::SessionStorage;
use entity::session_storage;
use crate::error::{Error, SessionError, ZenithError};

pub async fn find_session(session_id: i64, db: DatabaseConnection) -> Result<session_storage::Model, Error> {
    let result: Result<Option<session_storage::Model>, DbErr> = SessionStorage::find_by_id(session_id).one(&db).await;
    if result.is_ok() {
        let result = result.unwrap();
        if result.is_none() {
            Err(SessionError::SessionNotFound.error())
        } else {
            let result = result.unwrap();
            if expired(result.clone()).await { // Check Session is Expired
                Err(SessionError::SessionExpired.error())
            } else {
                Ok(result)
            }
        }
    } else {
        Err(SessionError::SessionNotFound.error())
    }
}

pub async fn expired(session: session_storage::Model) -> bool {
    if session.expire < Utc::now().naive_local() {
        true
    } else {
        false
    }
}

pub async fn session_owned(session_id: i64, user_id: i64, db: DatabaseConnection) {
    let result = find_session(session_id, db.clone()).await;
    if result.is_ok() {
        let mut result: session_storage::ActiveModel = result.unwrap().into();
        result.user_id = Set(Some(user_id.to_owned()));
        result.update(&db).await.unwrap();
    }
}

pub async fn extend_time(session_id: i64, db: DatabaseConnection) {
    let result = find_session(session_id, db.clone()).await;
    if result.is_ok() {
        let mut result: session_storage::ActiveModel = result.unwrap().into();
        result.expire = Set((Utc::now() + Duration::days(7)).naive_local().to_owned());
        result.update(&db).await.unwrap();
    }
}