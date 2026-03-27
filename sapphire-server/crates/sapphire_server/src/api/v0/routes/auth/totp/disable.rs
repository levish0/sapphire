use crate::extractors::RequiredSession;
use crate::service::auth::totp::service_totp_disable;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use sapphire_dto::auth::request::TotpDisableRequest;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/totp/disable",
    request_body = TotpDisableRequest,
    responses(
        (status = 204, description = "TOTP disabled"),
        (status = 400, description = "Invalid TOTP code or TOTP not enabled", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth - TOTP"
)]
pub async fn totp_disable(
    State(state): State<AppState>,
    RequiredSession(session): RequiredSession,
    ValidatedJson(payload): ValidatedJson<TotpDisableRequest>,
) -> Result<StatusCode, Errors> {
    service_totp_disable(&state.db, session.user_id, &payload.code).await?;
    Ok(StatusCode::NO_CONTENT)
}
