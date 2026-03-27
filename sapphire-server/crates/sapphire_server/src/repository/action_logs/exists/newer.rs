use super::super::filter::{ActionLogFilter, apply_action_log_filter};
use crate::repository::common::repository_query_exists;
use sapphire_entity::action_logs::{Column as ActionLogColumn, Entity as ActionLogEntity};
use sapphire_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

///
///
/// - `service_get_action_logs`
/// - `repository_query_exists`
///
/// # Errors
pub async fn repository_exists_newer_action_log<C>(
    conn: &C,
    filter: &ActionLogFilter,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let query = apply_action_log_filter(
        ActionLogEntity::find().filter(ActionLogColumn::Id.gt(cursor_id)),
        filter,
    );

    repository_query_exists(conn, query).await
}
