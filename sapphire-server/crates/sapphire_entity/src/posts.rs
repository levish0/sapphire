use sea_orm::prelude::*;
use uuid::Uuid;

use super::comments::Entity as CommentsEntity;
use super::users::Entity as UsersEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub user_id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    #[sea_orm(nullable)]
    pub media_urls: Option<Vec<String>>,
    #[sea_orm(nullable)]
    pub repost_of_id: Option<Uuid>,
    #[sea_orm(nullable)]
    pub quote_of_id: Option<Uuid>,
    #[sea_orm(not_null)]
    pub like_count: i32,
    #[sea_orm(not_null)]
    pub repost_count: i32,
    #[sea_orm(not_null)]
    pub quote_count: i32,
    #[sea_orm(not_null)]
    pub comment_count: i32,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::RepostOfId",
        to = "Column::Id",
        on_delete = "SetNull"
    )]
    RepostOf,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::QuoteOfId",
        to = "Column::Id",
        on_delete = "SetNull"
    )]
    QuoteOf,
    #[sea_orm(has_many = "CommentsEntity")]
    Comments,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<CommentsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
