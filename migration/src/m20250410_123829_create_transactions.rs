use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BankTransaction::Table)
                    .if_not_exists()
                    .col(pk_auto(BankTransaction::Id))
                    .col(date(BankTransaction::BookkeepingDate))
                    .col(string(BankTransaction::TransactionText))
                    .col(string(BankTransaction::Reference))
                    .col(string(BankTransaction::Hash))
                    .col(money(BankTransaction::Amount))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BankTransaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BankTransaction {
    Table,
    Id,
    BookkeepingDate,
    TransactionText,
    Reference,
    Hash,
    Amount,
}
