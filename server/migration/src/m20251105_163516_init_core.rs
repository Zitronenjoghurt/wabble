use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(string(User::Name))
                    .col(string(User::PasswordHash))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserSession::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserSession::Id))
                    .col(uuid(UserSession::UserId))
                    .col(string(UserSession::TokenHash))
                    .col(timestamp(UserSession::ExpiresAt))
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserSession::Table, UserSession::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(InviteCode::Table)
                    .if_not_exists()
                    .col(pk_uuid(InviteCode::Code))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserSession::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(InviteCode::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    PasswordHash,
}

#[derive(DeriveIden)]
enum UserSession {
    Table,
    Id,
    UserId,
    TokenHash,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum InviteCode {
    Table,
    Code,
}
