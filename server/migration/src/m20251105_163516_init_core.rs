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
                    .col(uuid_uniq(User::InviteCode))
                    .col(string_uniq(User::Name))
                    .col(string(User::PasswordHash))
                    .col(big_integer(User::Permissions))
                    .col(string_uniq(User::FriendCode))
                    .col(timestamp(User::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(User::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserSession::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserSession::UserId))
                    .col(string(UserSession::TokenHash))
                    .col(timestamp(UserSession::CreatedAt).default(Expr::current_timestamp()))
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
                    .table(UserFriendship::Table)
                    .if_not_exists()
                    .col(uuid(UserFriendship::User1Id))
                    .col(uuid(UserFriendship::User2Id))
                    .col(small_integer(UserFriendship::Status))
                    .col(timestamp(UserFriendship::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(UserFriendship::AcceptedAt).default(Expr::null()))
                    .primary_key(
                        Index::create()
                            .col(UserFriendship::User1Id)
                            .col(UserFriendship::User2Id),
                    )
                    .check(
                        Expr::col(UserFriendship::User1Id).lt(Expr::col(UserFriendship::User2Id)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserFriendship::Table, UserFriendship::User1Id)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserFriendship::Table, UserFriendship::User2Id)
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
                    .col(timestamp(InviteCode::CreatedAt).default(Expr::current_timestamp()))
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
            .drop_table(Table::drop().table(UserFriendship::Table).to_owned())
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
    InviteCode,
    Name,
    PasswordHash,
    Permissions,
    FriendCode,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserSession {
    Table,
    UserId,
    TokenHash,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum UserFriendship {
    Table,
    User1Id,
    User2Id,
    Status,
    CreatedAt,
    AcceptedAt,
}

#[derive(DeriveIden)]
enum InviteCode {
    Table,
    Code,
    CreatedAt,
}
