use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(InsertStocks::Table)
                    .col(pk_auto(InsertStocks::Id))
                    .col(integer(InsertStocks::ItemId))
                    .col(integer(InsertStocks::Value))
                    .col(integer(InsertStocks::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-insert_stocks-items")
                            .from(InsertStocks::Table, InsertStocks::ItemId)
                            .to(Items::Table, Items::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-insert_stocks-users")
                            .from(InsertStocks::Table, InsertStocks::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InsertStocks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum InsertStocks {
    Table,
    Id,
    ItemId,
    Value,
    UserId,
    
}


#[derive(DeriveIden)]
enum Items {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
