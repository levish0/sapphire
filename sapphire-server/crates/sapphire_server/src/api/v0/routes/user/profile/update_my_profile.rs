use crate::extractors::RequiredSession;
use crate::service::user::profile::update_my_profile::service_update_my_profile;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::user::UserResponse;
use sapphire_dto::user::request::UpdateMyProfileRequest;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    patch,
    path = "/v0/user/me",
    request_body = UpdateMyProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UserResponse),
        (status = 400, description = "Bad Request - Validation error", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User",
)]
pub async fn update_my_profile(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<UpdateMyProfileRequest>,
) -> Result<UserResponse, Errors> {
    service_update_my_profile(&state.db, &state.worker, &session_context, payload).await
}
