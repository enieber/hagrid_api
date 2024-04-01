#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20240330_221335_roles;
mod m20240330_221436_classifications;
mod m20240330_221556_items;
mod m20240401_184237_insert_stocks;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20240330_221335_roles::Migration),
            Box::new(m20240330_221436_classifications::Migration),
            Box::new(m20240330_221556_items::Migration),
            Box::new(m20240401_184237_insert_stocks::Migration),
        ]
    }
}