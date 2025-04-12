use std::fmt::Display;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Member::Table)
                    .if_not_exists()
                    .col(pk_uuid(Member::Id))
                    .col(string(Member::FirstName))
                    .col(string(Member::LastName))
                    .col(string_null(Member::Email))
                    .col(string_null(Member::MobilePhone))
                    .col(date_null(Member::BirthDate))
                    .col(string(Member::Hash))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Member::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden, Copy, Clone)]
pub enum Member {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    MobilePhone,
    BirthDate,
    Hash,
}

impl Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Member::Table => write!(f, "member_table"),
            Member::Id => write!(f, "member_id"),
            Member::FirstName => write!(f, "member_first_name"),
            Member::LastName => write!(f, "member_last_name"),
            Member::Email => write!(f, "member_email"),
            Member::MobilePhone => write!(f, "member_mobile_phone"),
            Member::BirthDate => write!(f, "member_birth_date"),
            Member::Hash => write!(f, "member_hash"),
        }
    }
}
