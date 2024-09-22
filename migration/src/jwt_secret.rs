use sea_orm_migration::{prelude::*};
use crate::user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum JWTSecret {
    Table,
    UserId,
    UserSecret,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JWTSecret::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(JWTSecret::UserId)
                        .big_integer()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(JWTSecret::UserSecret).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(JWTSecret::Table, JWTSecret::UserId)
                            .to(User::Table, User::UserId)
                    )
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(JWTSecret::Table).to_owned())
            .await
    }
}