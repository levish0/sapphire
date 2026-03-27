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
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(Posts::UserId).uuid().not_null())
                    .col(ColumnDef::new(Posts::Content).text().null())
                    .col(
                        ColumnDef::new(Posts::MediaUrls)
                            .array(ColumnType::Text)
                            .null(),
                    )
                    .col(ColumnDef::new(Posts::RepostOfId).uuid().null())
                    .col(ColumnDef::new(Posts::QuoteOfId).uuid().null())
                    .col(
                        ColumnDef::new(Posts::LikeCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Posts::RepostCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Posts::QuoteCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Posts::CommentCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Posts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_user")
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_repost_of")
                            .from(Posts::Table, Posts::RepostOfId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_quote_of")
                            .from(Posts::Table, Posts::QuoteOfId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .check(Expr::cust(
                        "NOT (repost_of_id IS NOT NULL AND quote_of_id IS NOT NULL)",
                    ))
                    .check(Expr::cust(
                        "content IS NOT NULL OR repost_of_id IS NOT NULL",
                    ))
                    .to_owned(),
            )
            .await?;

        // Supports: WHERE user_id = ? ORDER BY id DESC
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_user_id")
                    .table(Posts::Table)
                    .col(Posts::UserId)
                    .col((Posts::Id, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_repost_of_id")
                    .table(Posts::Table)
                    .col(Posts::RepostOfId)
                    .cond_where(Expr::col(Posts::RepostOfId).is_not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_quote_of_id")
                    .table(Posts::Table)
                    .col(Posts::QuoteOfId)
                    .cond_where(Expr::col(Posts::QuoteOfId).is_not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Posts {
    Table,
    Id,
    UserId,
    Content,
    MediaUrls,
    RepostOfId,
    QuoteOfId,
    LikeCount,
    RepostCount,
    QuoteCount,
    CommentCount,
    CreatedAt,
}
