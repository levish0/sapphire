use crate::extractors::RequiredSession;
use crate::service::user_preferences::service_set_user_preferences_bulk;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::user_preferences::{SetUserPreferencesBulkRequest, UserPreferencesResponse};
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    put,
    path = "/v0/preferences",
    request_body = SetUserPreferencesBulkRequest,
    responses(
        (status = 200, description = "User preferences set successfully", body = UserPreferencesResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Preferences"
)]
pub async fn set_user_preferences_bulk(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(request): ValidatedJson<SetUserPreferencesBulkRequest>,
) -> Result<UserPreferencesResponse, Errors> {
    service_set_user_preferences_bulk(&state.db, &session_context, request.preferences).await
}
