use sea_orm_migration::prelude::*;

use crate::m20230824_220114_create_image_table::Image;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Image::Table)
                    .add_column(ColumnDef::new(Image::PublicId).string().unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Image::Table)
                    .drop_column(Image::PublicId)
                    .to_owned(),
            )
            .await
    }
}
