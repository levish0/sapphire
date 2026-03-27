use crate::extractors::RequiredSession;
use crate::service::auth::logout::service_logout;
use crate::state::AppState;
use axum::{extract::State, response::Response};
use sapphire_dto::auth::response::create_logout_response;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/logout",
    responses(
        (status = 204, description = "Logout successful"),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Redis error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth"
)]
pub async fn auth_logout(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
) -> Result<Response, Errors> {
    service_logout(&state.redis_session, &session_context.session_id).await?;

    create_logout_response()
}
