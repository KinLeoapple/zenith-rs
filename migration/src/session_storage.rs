use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum SessionStorage {
    Table,
    SessionId,
    UserId,
    RsaPem,
    Expire
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SessionStorage::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SessionStorage::SessionId)
                        .big_integer()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(SessionStorage::RsaPem).string().null())
                    .col(ColumnDef::new(SessionStorage::UserId).big_integer().null())
                    .col(ColumnDef::new(SessionStorage::Expire).date_time().not_null())
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SessionStorage::Table).to_owned())
            .await
    }
}