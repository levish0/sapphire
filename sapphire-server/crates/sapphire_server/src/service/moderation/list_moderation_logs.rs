use crate::repository::moderation::{
    ModerationLogFilter, repository_exists_newer_moderation_log,
    repository_exists_older_moderation_log, repository_find_moderation_logs,
};
use crate::service::cursor_pagination::{edge_cursors, reverse_if_newer};
use sapphire_dto::moderation::{
    ListModerationLogsRequest, ListModerationLogsResponse, ModerationLogListItem,
};
use sapphire_dto::pagination::CursorDirection;
use sapphire_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;

pub async fn service_list_moderation_logs(
    db: &DatabaseConnection,
    payload: ListModerationLogsRequest,
) -> ServiceResult<ListModerationLogsResponse> {
    let limit = payload.limit;
    let is_newer = payload.cursor_direction == Some(CursorDirection::Newer);

    let filter = ModerationLogFilter {
        actor_id: payload.actor_id,
        resource_type: payload.resource_type,
        resource_id: payload.resource_id,
        actions: payload.actions,
        created_from: payload.created_from,
        created_to: payload.created_to,
    };

    let mut results = repository_find_moderation_logs(
        db,
        &filter,
        payload.cursor_id,
        payload.cursor_direction,
        limit,
    )
    .await?;

    let (has_newer, has_older) = if let Some((newer_cursor, older_cursor)) =
        edge_cursors(&results, is_newer, |result| result.id)
    {
        let has_newer = repository_exists_newer_moderation_log(db, &filter, newer_cursor).await?;
        let has_older = repository_exists_older_moderation_log(db, &filter, older_cursor).await?;
        (has_newer, has_older)
    } else {
        (false, false)
    };

    reverse_if_newer(&mut results, is_newer);

    let data: Vec<ModerationLogListItem> = results
        .into_iter()
        .map(ModerationLogListItem::from)
        .collect();

    Ok(ListModerationLogsResponse {
        data,
        has_newer,
        has_older,
    })
}
