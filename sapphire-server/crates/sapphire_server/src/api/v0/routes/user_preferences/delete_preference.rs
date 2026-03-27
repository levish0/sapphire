use crate::extractors::RequiredSession;
use crate::service::user_preferences::service_delete_user_preference;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use sapphire_dto::user_preferences::UserPreferenceKeyPath;
use sapphire_dto::validator::path_validator::ValidatedPath;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    delete,
    path = "/v0/preferences/{key}",
    params(UserPreferenceKeyPath),
    responses(
        (status = 204, description = "User preference deleted successfully"),
        (status = 400, description = "Bad request - Invalid preference key", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 404, description = "Preference not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Preferences"
)]
pub async fn delete_user_preference(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<UserPreferenceKeyPath>,
) -> Result<StatusCode, Errors> {
    service_delete_user_preference(&state.db, &session_context, path.key).await?;
    Ok(StatusCode::NO_CONTENT)
}
