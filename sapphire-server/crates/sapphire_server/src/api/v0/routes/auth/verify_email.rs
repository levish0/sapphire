use crate::bridge::worker_client;
use crate::service::auth::verify_email::service_verify_email;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sapphire_dto::auth::request::VerifyEmailRequest;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/verify-email",
    request_body = VerifyEmailRequest,
    responses(
        (status = 204, description = "Email verified successfully"),
        (status = 400, description = "Bad request - Invalid JSON, validation error, invalid or expired token", body = ErrorResponse),
        (status = 404, description = "Not Found - User not found", body = ErrorResponse),
        (status = 409, description = "Conflict - Email already verified", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database or Redis error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn auth_verify_email(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<VerifyEmailRequest>,
) -> Result<impl IntoResponse, Errors> {
    let user_id = service_verify_email(&state.db, &state.redis_session, &payload.token).await?;

    worker_client::index_user(&state.worker, user_id).await.ok();

    Ok(StatusCode::NO_CONTENT)
}
