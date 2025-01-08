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
                    .table(Members::Table)
                    .if_not_exists()
                    .col(pk_auto(Members::Id))
                    .col(string(Members::FirstName))
                    .col(string(Members::LastName))
                    .col(string(Members::Email))
                    .col(string(Members::MobilePhone))
                    .col(date(Members::BirthDate))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Members::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Members {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    MobilePhone,
    BirthDate,
}
