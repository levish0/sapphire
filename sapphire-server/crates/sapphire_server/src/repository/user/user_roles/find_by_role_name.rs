use chrono::Utc;
use sapphire_entity::common::Role;
use sapphire_entity::user_roles::{Column, Entity};
use sapphire_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_active_user_ids_by_role_name<C>(
    conn: &C,
    role: Role,
) -> Result<Vec<Uuid>, Errors>
where
    C: ConnectionTrait,
{
    let now = Utc::now();

    let roles = Entity::find()
        .filter(Column::Role.eq(role))
        .filter(Column::ExpiresAt.is_null().or(Column::ExpiresAt.gt(now)))
        .all(conn)
        .await?;

    Ok(roles.into_iter().map(|r| r.user_id).collect())
}
