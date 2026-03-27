use crate::extractors::RequiredSession;
use crate::service::user_preferences::service_get_user_preferences;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::user_preferences::UserPreferencesResponse;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    get,
    path = "/v0/preferences",
    responses(
        (status = 200, description = "User preferences retrieved successfully", body = UserPreferencesResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Preferences"
)]
pub async fn get_user_preferences(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
) -> Result<UserPreferencesResponse, Errors> {
    service_get_user_preferences(&state.db, &session_context).await
}
