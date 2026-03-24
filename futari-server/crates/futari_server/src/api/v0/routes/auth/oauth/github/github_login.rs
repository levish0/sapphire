use crate::middleware::anonymous_user::AnonymousUserContext;
use crate::service::oauth::github::service_github_sign_in;
use crate::state::AppState;
use crate::utils::extract::extract_user_agent::extract_user_agent;
use axum::Extension;
use axum::{extract::State, response::Response};
use axum_extra::{TypedHeader, headers::UserAgent};
use futari_dto::oauth::request::github::GithubLoginRequest;
use futari_dto::oauth::response::{OAuthPendingSignupResponse, OAuthSignInResponse};
use futari_dto::validator::json_validator::ValidatedJson;
use futari_errors::errors::{ErrorResponse, Errors};

///
#[utoipa::path(
    post,
    path = "/v0/auth/oauth/github/login",
    request_body = GithubLoginRequest,
    responses(
        (status = 200, description = "New user - pending signup required", body = OAuthPendingSignupResponse),
        (status = 204, description = "Login successful (existing user)"),
        (status = 400, description = "Bad request - Invalid JSON, validation error, invalid/expired state/code, or no verified email", body = ErrorResponse),
        (status = 409, description = "Conflict - Email already exists", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database, Redis, or OAuth provider error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn auth_github_login(
    user_agent: Option<TypedHeader<UserAgent>>,
    State(state): State<AppState>,
    Extension(anonymous): Extension<AnonymousUserContext>,
    ValidatedJson(payload): ValidatedJson<GithubLoginRequest>,
) -> Result<Response, Errors> {
    let user_agent_str = extract_user_agent(user_agent);

    let result = service_github_sign_in(
        &state.db,
        &state.redis_session,
        &state.http_client,
        &payload.code,
        &payload.state,
        &anonymous.anonymous_user_id,
        Some(user_agent_str),
    )
    .await?;

    OAuthSignInResponse::from_result(result).into_response_result()
}
