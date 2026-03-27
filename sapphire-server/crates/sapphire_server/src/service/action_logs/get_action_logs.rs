use crate::repository::action_logs::{
    ActionLogFilter, repository_exists_newer_action_log, repository_exists_older_action_log,
    repository_find_action_logs,
};
use crate::service::cursor_pagination::{edge_cursors, reverse_if_newer};
use sapphire_dto::action_logs::{ActionLogListResponse, ActionLogResponse, GetActionLogsRequest};
use sapphire_dto::pagination::CursorDirection;
use sapphire_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;

/// Fetch action logs with sapphire's current action/resource filter model.
pub async fn service_get_action_logs(
    db: &DatabaseConnection,
    payload: GetActionLogsRequest,
) -> ServiceResult<ActionLogListResponse> {
    let limit = payload.limit;
    let is_newer = payload.cursor_direction == Some(CursorDirection::Newer);

    let filter = ActionLogFilter {
        actor_id: payload.user_id,
        resource_type: payload.resource_type,
        resource_id: payload.resource_id,
        actions: payload.actions,
        created_from: payload.created_from,
        created_to: payload.created_to,
    };

    let mut logs = repository_find_action_logs(
        db,
        &filter,
        payload.cursor_id,
        payload.cursor_direction,
        limit,
    )
    .await?;

    let (has_newer, has_older) =
        if let Some((newer_cursor, older_cursor)) = edge_cursors(&logs, is_newer, |log| log.id) {
            let has_newer = repository_exists_newer_action_log(db, &filter, newer_cursor).await?;
            let has_older = repository_exists_older_action_log(db, &filter, older_cursor).await?;
            (has_newer, has_older)
        } else {
            (false, false)
        };

    reverse_if_newer(&mut logs, is_newer);

    let data: Vec<ActionLogResponse> = logs.into_iter().map(ActionLogResponse::from).collect();

    Ok(ActionLogListResponse {
        data,
        has_newer,
        has_older,
    })
}
