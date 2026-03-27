use crate::service::auth::resend_verification_email::service_resend_verification_email;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sapphire_dto::auth::request::ResendVerificationEmailRequest;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/resend-verification-email",
    request_body = ResendVerificationEmailRequest,
    responses(
        (status = 204, description = "Verification email sent if a pending signup exists"),
        (status = 400, description = "Bad request - Invalid JSON or validation error", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Redis error", body = ErrorResponse),
        (status = 502, description = "Bad Gateway - Worker service request failed or returned invalid response", body = ErrorResponse),
        (status = 503, description = "Service Unavailable - Worker service connection failed", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn auth_resend_verification_email(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<ResendVerificationEmailRequest>,
) -> Result<impl IntoResponse, Errors> {
    service_resend_verification_email(&state.redis_session, &state.worker, &payload.email).await?;

    Ok(StatusCode::NO_CONTENT)
}
