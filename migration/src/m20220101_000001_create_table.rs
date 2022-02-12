use sea_schema::migration::{
    sea_query::{self, *},
    *,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}
#[derive(Iden)]
pub enum User {
    Table,
    UserName,
    Email,
    Password,
    FirstName,
    LastName,
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::UserName)
                        .string()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(User::Email).string().not_null())
                .col(ColumnDef::new(User::Password).string().not_null())
                .col(ColumnDef::new(User::FirstName).string().not_null())
                .col(ColumnDef::new(User::LastName).string().not_null())
                .to_owned()
    )
    .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}
