use crate::service::moderation::service_list_moderation_logs;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::moderation::{ListModerationLogsRequest, ListModerationLogsResponse};
use sapphire_dto::validator::query_validator::ValidatedQuery;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    get,
    path = "/v0/moderation/logs",
    params(ListModerationLogsRequest),
    responses(
        (status = 200, description = "Moderation logs retrieved successfully", body = ListModerationLogsResponse),
        (status = 400, description = "Bad request - Invalid query parameters or validation error", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database error", body = ErrorResponse)
    ),
    tag = "Moderation"
)]
pub async fn list_moderation_logs(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<ListModerationLogsRequest>,
) -> Result<ListModerationLogsResponse, Errors> {
    service_list_moderation_logs(&state.db, payload).await
}
