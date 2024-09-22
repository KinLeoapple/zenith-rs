use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum User {
    Table,
    UserId,
    UserDescription,
    UserName,
    UserPassword,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::UserId)
                        .big_integer()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(User::UserDescription).string().not_null())
                    .col(ColumnDef::new(User::UserName)
                        .string()
                        .not_null()
                        .unique_key()
                    )
                    .col(ColumnDef::new(User::UserPassword).string().not_null())
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}