use sea_orm_migration::prelude::*;

use crate::{m20230824_110340_create_user_table::User, m20230824_220114_create_image_table::Image};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Profile::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Profile::UserId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Profile::UserName).string().unique_key())
                    .col(ColumnDef::new(Profile::Bio).string())
                    .col(ColumnDef::new(Profile::ImageId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_profile_user")
                            .from(Profile::Table, Profile::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_profile_image")
                            .from(Profile::Table, Profile::ImageId)
                            .to(Image::Table, Image::Id)
                            .on_update(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Profile {
    Table,
    Id,
    UserId,
    UserName,
    Bio,
    ImageId,
}
