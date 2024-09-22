use sea_orm_migration::{prelude::*};
use crate::user::User;
use crate::category::Category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Blog {
    Table,
    BlogId,
    UserId,
    CategoryId,
    BlogTitle,
    BlogDescription,
    BlogPubDt,
    IsDraft
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Blog::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Blog::BlogId)
                        .big_integer()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Blog::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Blog::CategoryId).big_integer().not_null())
                    .col(ColumnDef::new(Blog::BlogTitle).string().not_null())
                    .col(ColumnDef::new(Blog::BlogDescription).string().not_null())
                    .col(ColumnDef::new(Blog::BlogPubDt).date_time().not_null())
                    .col(ColumnDef::new(Blog::IsDraft).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Blog::Table, Blog::UserId)
                            .to(User::Table, User::UserId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Blog::Table, Blog::BlogId)
                            .to(Category::Table, Category::CategoryId)
                    )
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Blog::Table).to_owned())
            .await
    }
}
