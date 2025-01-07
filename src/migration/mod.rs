pub use sea_orm_migration::prelude::*;

mod m20220120_000001_create_post_table;
mod m20241205_170802_create_members_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220120_000001_create_post_table::Migration),
            Box::new(m20241205_170802_create_members_table::Migration),
        ]
    }
}
