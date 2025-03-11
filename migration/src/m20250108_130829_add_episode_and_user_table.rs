use crate::foreign_key_auto;
use sea_orm_migration::{prelude::*, schema::*};
use std::fmt;
use std::fmt::Display;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(string(User::Email))
                    .col(string(User::Name))
                    .col(string(User::Role).default("user"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(foreign_key_auto(
                &mut Table::create()
                    .table(Episode::Table)
                    .if_not_exists()
                    .col(pk_uuid(Episode::Id))
                    .col(string(Episode::Title))
                    .col(string(Episode::Summary))
                    .col(string(Episode::Tags))
                    .col(string_null(Episode::Url))
                    .to_owned(),
                Episode::Table,
                Episode::UserId,
                User::Table,
                User::Id,
                true,
            ))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Episode::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden, Copy, Clone)]
enum Episode {
    Table,
    Id,
    UserId,
    Title,
    Summary,
    Tags,
    Url,
}
impl Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Episode::Table => write!(f, "episode_table"),
            Episode::Id => write!(f, "episode_id"),
            Episode::UserId => write!(f, "episode_user_id"),
            Episode::Title => write!(f, "episode_title"),
            Episode::Summary => write!(f, "episode_summary"),
            Episode::Tags => write!(f, "episode_tags"),
            Episode::Url => write!(f, "episode_url"),
        }
    }
}

#[derive(DeriveIden, Copy, Clone)]
enum User {
    Table,
    Id,
    Email,
    Name,
    Role,
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            User::Table => write!(f, "user_table"),
            User::Id => write!(f, "user_id"),
            User::Name => write!(f, "user_name"),
            User::Email => write!(f, "user_email"),
            User::Role => write!(f, "user_role"),
        }
    }
}
