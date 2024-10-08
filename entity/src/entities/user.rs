//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
    pub user_description: String,
    #[sea_orm(unique)]
    pub user_name: String,
    pub user_password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::blog::Entity")]
    Blog,
    #[sea_orm(has_many = "super::jwt_secret::Entity")]
    JwtSecret,
}

impl Related<super::blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl Related<super::jwt_secret::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JwtSecret.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
