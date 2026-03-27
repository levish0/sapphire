use crate::extractors::RequiredSession;
use crate::service::auth::change_password::service_change_password;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sapphire_dto::auth::request::ChangePasswordRequest;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/change-password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 204, description = "Password changed successfully"),
        (status = 400, description = "Bad request - Invalid JSON, validation error, or incorrect password", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth"
)]
pub async fn auth_change_password(
    State(state): State<AppState>,
    RequiredSession(session): RequiredSession,
    ValidatedJson(payload): ValidatedJson<ChangePasswordRequest>,
) -> Result<impl IntoResponse, Errors> {
    service_change_password(
        &state.db,
        &state.redis_session,
        session.user_id,
        &session.session_id,
        payload,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
