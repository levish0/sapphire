use crate::repository::document::revisions::{
    repository_exists_newer_revision_for_author, repository_exists_older_revision_for_author,
    repository_find_revisions_by_author_id_cursor,
};
use crate::repository::user::get_by_handle::repository_get_user_by_handle;
use crate::service::cursor_pagination::{edge_cursors, reverse_if_newer};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use sapphire_dto::pagination::CursorDirection;
use sapphire_dto::user::response::revision_contributions::{
    GetUserRevisionContributionsResponse, UserRevisionContributionItem,
};
use sapphire_errors::errors::ServiceResult;

///
///
/// - `repository_get_user_by_handle`
/// - `repository_find_revisions_by_author_id_cursor`
/// - `repository_exists_newer_revision_for_author`
/// - `repository_exists_older_revision_for_author`
///
/// # Errors
pub async fn service_get_user_revision_contributions(
    db: &DatabaseConnection,
    handle: &str,
    cursor: Option<Uuid>,
    direction: Option<CursorDirection>,
    limit: u32,
) -> ServiceResult<GetUserRevisionContributionsResponse> {
    // Find user by handle to get user_id
    let user = repository_get_user_by_handle(db, handle.to_string()).await?;

    // Track direction for result ordering
    let is_newer = direction == Some(CursorDirection::Newer);

    // Policy: public contributions list excludes hidden revisions and
    // revisions belonging to hidden documents.
    // The same filter is also applied in has_newer/has_older checks.
    // (See repository/document/revisions/filter.rs)
    // Fetch revisions with cursor-based pagination
    let mut revisions =
        repository_find_revisions_by_author_id_cursor(db, user.id, cursor, direction, limit as u64)
            .await?;

    let (has_newer, has_older) = if let Some((newer_cursor, older_cursor)) =
        edge_cursors(&revisions, is_newer, |revision| revision.revision_id)
    {
        let has_newer =
            repository_exists_newer_revision_for_author(db, user.id, newer_cursor).await?;
        let has_older =
            repository_exists_older_revision_for_author(db, user.id, older_cursor).await?;
        (has_newer, has_older)
    } else {
        (false, false)
    };

    reverse_if_newer(&mut revisions, is_newer);

    // Convert to response format
    let items: Vec<UserRevisionContributionItem> = revisions
        .into_iter()
        .map(|r| UserRevisionContributionItem {
            revision_id: r.revision_id,
            revision_number: r.revision_number,
            edit_summary: r.edit_summary,
            content_length: r.content_length,
            content_chars_added: r.content_chars_added,
            content_chars_removed: r.content_chars_removed,
            hidden_at: r.hidden_at,
            created_at: r.created_at,
            document_id: r.document_id,
            document_namespace: r.document_namespace,
            document_title: r.document_title,
        })
        .collect();

    Ok(GetUserRevisionContributionsResponse {
        items,
        has_newer,
        has_older,
    })
}
