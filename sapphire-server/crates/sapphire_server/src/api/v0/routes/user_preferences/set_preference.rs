use crate::extractors::RequiredSession;
use crate::service::user_preferences::service_set_user_preference;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::user_preferences::{
    SetUserPreferenceRequest, UserPreferenceKeyPath, UserPreferenceResponse,
};
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_dto::validator::path_validator::ValidatedPath;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    put,
    path = "/v0/preferences/{key}",
    params(UserPreferenceKeyPath),
    request_body = SetUserPreferenceRequest,
    responses(
        (status = 200, description = "User preference set successfully", body = UserPreferenceResponse),
        (status = 400, description = "Bad request - Invalid JSON, validation error, or invalid preference key", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Preferences"
)]
pub async fn set_user_preference(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<UserPreferenceKeyPath>,
    ValidatedJson(request): ValidatedJson<SetUserPreferenceRequest>,
) -> Result<UserPreferenceResponse, Errors> {
    service_set_user_preference(&state.db, &session_context, path.key, request.value).await
}
