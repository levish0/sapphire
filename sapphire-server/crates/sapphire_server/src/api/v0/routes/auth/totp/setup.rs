use crate::extractors::RequiredSession;
use crate::service::auth::totp::service_totp_setup;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::auth::response::TotpSetupResponse;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/totp/setup",
    responses(
        (status = 200, description = "TOTP setup initiated", body = TotpSetupResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 409, description = "TOTP already enabled", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth - TOTP"
)]
pub async fn totp_setup(
    State(state): State<AppState>,
    RequiredSession(session): RequiredSession,
) -> Result<TotpSetupResponse, Errors> {
    service_totp_setup(&state.db, session.user_id).await
}
