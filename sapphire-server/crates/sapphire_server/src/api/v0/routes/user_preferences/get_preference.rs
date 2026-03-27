use crate::extractors::RequiredSession;
use crate::service::user_preferences::service_get_user_preference;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::user_preferences::{UserPreferenceKeyPath, UserPreferenceResponse};
use sapphire_dto::validator::path_validator::ValidatedPath;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    get,
    path = "/v0/preferences/{key}",
    params(UserPreferenceKeyPath),
    responses(
        (status = 200, description = "User preference retrieved successfully", body = UserPreferenceResponse),
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
pub async fn get_user_preference(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<UserPreferenceKeyPath>,
) -> Result<UserPreferenceResponse, Errors> {
    service_get_user_preference(&state.db, &session_context, path.key).await
}
