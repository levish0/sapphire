use super::filter::{ModerationLogFilter, apply_moderation_log_filter};
use sapphire_dto::pagination::CursorDirection;
use sapphire_entity::moderation_logs::{
    Column as ModerationLogColumn, Entity as ModerationLogEntity, Model as ModerationLogModel,
};
use sapphire_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

///
///
/// - `service_list_moderation_logs`
/// - `apply_moderation_log_filter`
///
/// # Errors
pub async fn repository_find_moderation_logs<C>(
    conn: &C,
    filter: &ModerationLogFilter,
    cursor_id: Option<Uuid>,
    cursor_direction: Option<CursorDirection>,
    limit: u64,
) -> Result<Vec<ModerationLogModel>, Errors>
where
    C: ConnectionTrait,
{
    let mut query = apply_moderation_log_filter(ModerationLogEntity::find(), filter);

    // Apply cursor-based filtering (UUIDv7 is time-sortable)
    if let Some(id) = cursor_id {
        let direction = cursor_direction.unwrap_or(CursorDirection::Older);
        query = match direction {
            CursorDirection::Older => query
                .filter(ModerationLogColumn::Id.lt(id))
                .order_by_desc(ModerationLogColumn::Id),
            CursorDirection::Newer => query
                .filter(ModerationLogColumn::Id.gt(id))
                .order_by_asc(ModerationLogColumn::Id),
        };
    } else {
        query = query.order_by_desc(ModerationLogColumn::Id);
    }

    let results = query.limit(limit).all(conn).await?;

    Ok(results)
}
