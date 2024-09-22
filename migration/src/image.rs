use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Image {
    Table,
    ImageId,
    ImageHash,
    ImagePubDt
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Image::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Image::ImageId)
                        .big_integer()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Image::ImageHash)
                        .string()
                        .not_null()
                        .unique_key()
                    )
                    .col(ColumnDef::new(Image::ImagePubDt).date_time().not_null())
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Image::Table).to_owned())
            .await
    }
}