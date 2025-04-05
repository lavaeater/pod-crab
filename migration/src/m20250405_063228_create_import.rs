use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Import::Table)
                    .if_not_exists()
                    .col(pk_auto(Import::Id))
                    .col(string(Import::Title))
                    .col(string(Import::Text))
                    .col(binary(Import::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Import::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Import {
    Table,
    Id,
    Title,
    Text,
    Data,
}
