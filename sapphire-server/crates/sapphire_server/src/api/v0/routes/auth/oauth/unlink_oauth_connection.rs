use crate::extractors::RequiredSession;
use crate::service::oauth::unlink_connection::service_unlink_oauth;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use sapphire_dto::oauth::request::unlink::UnlinkOAuthRequest;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/oauth/connections/unlink",
    request_body = UnlinkOAuthRequest,
    responses(
        (status = 204, description = "OAuth unlinked successfully"),
        (status = 400, description = "Bad request - Invalid JSON, validation error, or cannot unlink last authentication method", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 404, description = "Not Found - User or OAuth connection not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database error", body = ErrorResponse)
    ),
    tag = "Auth",
    security(
        ("session_id_cookie" = [])
    )
)]
pub async fn unlink_oauth_connection(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<UnlinkOAuthRequest>,
) -> Result<StatusCode, Errors> {
    service_unlink_oauth(&state.db, session_context.user_id, payload.provider).await?;

    Ok(StatusCode::NO_CONTENT)
}
