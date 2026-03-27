use chrono::Utc;
use sapphire_entity::user_roles::{Column, Entity, Model};
use sapphire_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_user_role_entries<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<Vec<Model>, Errors>
where
    C: ConnectionTrait,
{
    let now = Utc::now();

    let entries = Entity::find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::ExpiresAt.is_null().or(Column::ExpiresAt.gt(now)))
        .all(conn)
        .await?;

    Ok(entries)
}
