use crate::service::auth::totp::service_totp_verify;
use crate::state::AppState;
use axum::extract::State;
use axum::response::Response;
use sapphire_dto::auth::request::TotpVerifyRequest;
use sapphire_dto::auth::response::create_login_response;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/totp/verify",
    request_body = TotpVerifyRequest,
    responses(
        (status = 204, description = "TOTP verified, login successful"),
        (status = 400, description = "Invalid TOTP code or temp token", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    tag = "Auth - TOTP"
)]
pub async fn totp_verify(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<TotpVerifyRequest>,
) -> Result<Response, Errors> {
    let result = service_totp_verify(
        &state.db,
        &state.redis_session,
        &payload.temp_token,
        &payload.code,
    )
    .await?;

    create_login_response(result.session_id, result.remember_me)
}
