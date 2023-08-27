use chrono::{Duration, Utc};
use sea_orm_migration::prelude::*;
use uuid::Uuid;

use crate::m20230824_110340_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let def_uuid = Some(Box::new(Uuid::new_v4()));
        let def_expiration = Some(Box::new(Utc::now() + Duration::hours(24)));
        manager
            .create_table(
                Table::create()
                    .table(Token::Table)
                    .if_not_exists()
                    .comment("A users access/auth token")
                    .col(
                        ColumnDef::new(Token::Id)
                            .uuid()
                            .unique_key()
                            .not_null()
                            .primary_key()
                            .default(Value::Uuid(def_uuid)),
                    )
                    .col(
                        ColumnDef::new(Token::UserId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Token::Token).string().not_null())
                    .col(
                        ColumnDef::new(Token::ExpiresAt)
                            .date_time()
                            .not_null()
                            .default(Value::ChronoDateTimeUtc(def_expiration)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_token-user")
                            .from(Token::Table, Token::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Token::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Token {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
}
