use std::fmt::Display;
use crate::foreign_key_auto;
use sea_orm_migration::{prelude::*, schema::*};
use crate::m20241205_170802_create_member_table::Member;
use crate::sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                foreign_key_auto(
                    &mut Table::create()
                        .table(MemberEvent::Table)
                        .if_not_exists()
                        .col(pk_uuid(MemberEvent::Id))
                        .col(enumeration(MemberEvent::Type, Alias::new("member_event_type"), MemberEventType::iter()))
                        .col(json(MemberEvent::Data))
                        .col(date(MemberEvent::HappenedAt))
                        .to_owned(),
                MemberEvent::Table,
                MemberEvent::MemberId,
                Member::Table,
                Member::Id,
                true,
            ))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MemberEvent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden, Copy, Clone)]
enum MemberEvent {
    Table,
    Id,
    Type,
    Data,
    HappenedAt,
    MemberId,
}

impl Display for MemberEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemberEvent::Table => write!(f, "member_event_table"),
            MemberEvent::Id => write!(f, "member_event_id"),
            MemberEvent::Type => write!(f, "member_event_type"),
            MemberEvent::Data => write!(f, "member_event_data"),
            MemberEvent::HappenedAt => write!(f, "member_event_happened_at"),
            MemberEvent::MemberId => write!(f, "member_event_member_id"),
        }
    }
}

#[derive(Iden, EnumIter, Copy, Clone)]
enum MemberEventType {
    #[iden = "payment"]
    Payment,
    #[iden = "other"]
    Other,
}

impl Display for MemberEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemberEventType::Payment => write!(f, "payment"),
            MemberEventType::Other => write!(f, "other"),
        }
    }
}