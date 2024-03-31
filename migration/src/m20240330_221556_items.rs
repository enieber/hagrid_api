use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Items::Table)
                    .col(pk_auto(Items::Id))
                    .col(string_null(Items::Code))
                    .col(integer(Items::RoleId))
                    .col(integer(Items::ClassificationId))
                    .col(timestamp_null(Items::ValidityDate))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-items-roles")
                            .from(Items::Table, Items::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-items-classifications")
                            .from(Items::Table, Items::ClassificationId)
                            .to(Classifications::Table, Classifications::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Items {
    Table,
    Id,
    Code,
    RoleId,
    ClassificationId,
    ValidityDate,
    
}


#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Classifications {
    Table,
    Id,
}
