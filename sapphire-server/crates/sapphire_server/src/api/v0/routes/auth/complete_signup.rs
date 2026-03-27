use crate::middleware::anonymous_user::AnonymousUserContext;
use crate::service::oauth::complete_signup::service_complete_signup;
use crate::state::AppState;
use crate::utils::extract::extract_user_agent::extract_user_agent;
use axum::Extension;
use axum::{extract::State, response::Response};
use axum_extra::{TypedHeader, headers::UserAgent};
use sapphire_dto::auth::request::CompleteSignupRequest;
use sapphire_dto::auth::response::create_login_response;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

///
#[utoipa::path(
    post,
    path = "/v0/auth/complete-signup",
    request_body = CompleteSignupRequest,
    responses(
        (status = 204, description = "Signup completed successfully"),
        (status = 400, description = "Bad request - Invalid JSON or validation error", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Token expired or invalid", body = ErrorResponse),
        (status = 409, description = "Conflict - Handle or email already exists", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database or Redis error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn auth_complete_signup(
    user_agent: Option<TypedHeader<UserAgent>>,
    State(state): State<AppState>,
    Extension(anonymous): Extension<AnonymousUserContext>,
    ValidatedJson(payload): ValidatedJson<CompleteSignupRequest>,
) -> Result<Response, Errors> {
    let user_agent_str = extract_user_agent(user_agent);

    let session_id = service_complete_signup(
        &state.db,
        &state.redis_session,
        &state.http_client,
        &state.r2_assets,
        &state.worker,
        &payload.pending_token,
        &payload.handle,
        &anonymous.anonymous_user_id,
        Some(user_agent_str),
    )
    .await?;

    // Return 204 with login cookie (session max lifetime is server-configured).
    create_login_response(session_id, true)
}
