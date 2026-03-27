use crate::m20260324_061602_users::Users;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserPreferences::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserPreferences::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(UserPreferences::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserPreferences::Key).text().not_null())
                    .col(
                        ColumnDef::new(UserPreferences::Value)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPreferences::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_preferences_user_id")
                            .from(UserPreferences::Table, UserPreferences::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (user_id, key)
        manager
            .create_index(
                Index::create()
                    .name("idx_user_preferences_unique")
                    .table(UserPreferences::Table)
                    .col(UserPreferences::UserId)
                    .col(UserPreferences::Key)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // (user_id, key) unique index also covers user_id-only lookups (leftmost prefix).
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPreferences::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserPreferences {
    Table,
    Id,
    UserId,
    Key,
    Value,
    UpdatedAt,
}
