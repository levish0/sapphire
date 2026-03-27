use super::super::filter::{ModerationLogFilter, apply_moderation_log_filter};
use crate::repository::common::repository_query_exists;
use sapphire_entity::moderation_logs::{
    Column as ModerationLogColumn, Entity as ModerationLogEntity,
};
use sapphire_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

///
///
/// - `service_list_moderation_logs`
/// - `repository_query_exists`
///
/// # Errors
pub async fn repository_exists_older_moderation_log<C>(
    conn: &C,
    filter: &ModerationLogFilter,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let query = apply_moderation_log_filter(
        ModerationLogEntity::find().filter(ModerationLogColumn::Id.lt(cursor_id)),
        filter,
    );

    repository_query_exists(conn, query).await
}
