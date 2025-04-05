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
                    .table(Import::Table)
                    .if_not_exists()
                    .col(pk_auto(Import::Id))
                    .col(string(Import::Title))
                    .col(string(Import::Text))
                    .col(binary(Import::Data))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(foreign_key_auto(
                &mut Table::create()
                    .table(ImportRow::Table)
                    .if_not_exists()
                    .col(pk_uuid(ImportRow::Id))
                    .col(string(ImportRow::Data))
                    .col(string(ImportRow::Hash))
                    .to_owned(),
                ImportRow::Table,
                ImportRow::ImportId,
                Import::Table,
                Import::Id,
                true,
            ))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Import::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden, Copy, Clone, Debug, Hash)]
enum Import {
    Table,
    Id,
    Title,
    Text,
    Data,
}

impl Display for Import {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Import::Table => write!(f, "import_table"),
            Import::Id => write!(f, "import_id"),
            Import::Title => write!(f, "import_title"),
            Import::Text => write!(f, "import_text"),
            Import::Data => write!(f, "import_data"),
        }
    }
}

#[derive(DeriveIden, Copy, Clone, Debug, Hash)]
enum ImportRow {
    Table,
    Id,
    ImportId,
    Data,
    Hash,
}

impl Display for ImportRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportRow::Table => write!(f, "import_row_table"),
            ImportRow::Id => write!(f, "import_row_id"),
            ImportRow::ImportId => write!(f, "import_row_import_id"),
            ImportRow::Data => write!(f, "import_row_data"),
            ImportRow::Hash => {
                write!(f, "import_row_hash")
            }
        }
    }
}
