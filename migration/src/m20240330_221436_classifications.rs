use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Classifications::Table)
                    .col(pk_auto(Classifications::Id))
                    .col(string_null(Classifications::Name))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Classifications::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Classifications {
    Table,
    Id,
    Name,
    
}


