pub use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::integer;
use std::fmt::Display;

mod m20220120_000001_create_post_table;
mod m20241205_170802_create_members_table;
mod m20250108_130829_add_episode_and_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220120_000001_create_post_table::Migration),
            Box::new(m20241205_170802_create_members_table::Migration),
            Box::new(m20250108_130829_add_episode_and_user_table::Migration),
        ]
    }
}

pub fn foreign_key_auto<T, U>(
    table_create_statement: &mut TableCreateStatement,
    from_table: T,
    fk_column: T,
    to_table: U,
    to_id_column: U,
) -> TableCreateStatement
where
    T: IntoIden + Copy + Display + 'static,
    U: IntoIden + Copy + Display + 'static,
{
    table_create_statement.col(integer(fk_column).not_null());
    table_create_statement.foreign_key(&mut fk_auto(from_table, fk_column, to_table, to_id_column));
    table_create_statement.to_owned()
}

pub fn fk_auto<T, U>(
    from_table: T,
    fk_column: T,
    to_table: U,
    to_id_column: U,
) -> ForeignKeyCreateStatement
where
    T: IntoIden + Copy + Display + 'static,
    U: IntoIden + Copy + Display + 'static,
{
    ForeignKey::create()
        .name(format!("fk_{}_{}", from_table, to_table))
        .from(from_table, fk_column)
        .to(to_table, to_id_column)
        .on_delete(ForeignKeyAction::Cascade)
        .on_update(ForeignKeyAction::Cascade)
        .take()
}
