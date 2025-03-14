use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Todo::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Todo::Title).string().not_null())
                    .col(
                        ColumnDef::new(Todo::Completed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Todo::LastUpdated)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).if_exists().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Title,
    Completed,
    LastUpdated,
}
