use chrono::Utc;
use sea_orm_migration::prelude::*;

use crate::m20230824_110340_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let def_date_added = Some(Box::new(Utc::now()));
        manager.create_table(
            Table::create().table(Image::Table)
                .if_not_exists()
                .col(ColumnDef::new(Image::Id)
                     .integer()
                     .not_null()
                     .auto_increment()
                     .primary_key())
                .col(ColumnDef::new(Image::Title)
                     .string())
                .col(ColumnDef::new(Image::URL)
                     .string()
                     .not_null())
                .col(ColumnDef::new(Image::Model)
                     .string())
                .col(ColumnDef::new(Image::DateAdded)
                     .date_time()
                     .not_null()
                     .default(Value::ChronoDateTimeUtc(def_date_added)))
                .col(ColumnDef::new(Image::UserId)
                     .integer())
                .foreign_key(ForeignKey::create()
                             .name("FK_image_user")
                             .from(Image::Table, Image::UserId)
                             .to(User::Table, User::Id)
                             .on_delete(ForeignKeyAction::SetNull)
                             .on_update(ForeignKeyAction::Cascade))
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(Image::Table).to_owned()
        ).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Image {
    Table,
    Id,
    URL,
    Title,
    Model,
    DateAdded,
    UserId,
    PublicId
}
