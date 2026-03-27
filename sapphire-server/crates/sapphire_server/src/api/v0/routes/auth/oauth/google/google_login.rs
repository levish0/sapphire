use crate::middleware::anonymous_user::AnonymousUserContext;
use crate::service::oauth::google::service_google_sign_in;
use crate::state::AppState;
use crate::utils::extract::extract_user_agent::extract_user_agent;
use axum::Extension;
use axum::{extract::State, response::Response};
use axum_extra::{TypedHeader, headers::UserAgent};
use sapphire_dto::oauth::request::google::GoogleLoginRequest;
use sapphire_dto::oauth::response::{OAuthPendingSignupResponse, OAuthSignInResponse};
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

///
#[utoipa::path(
    post,
    path = "/v0/auth/oauth/google/login",
    request_body = GoogleLoginRequest,
    responses(
        (status = 200, description = "New user - pending signup required", body = OAuthPendingSignupResponse),
        (status = 204, description = "Login successful (existing user)"),
        (status = 400, description = "Bad request - Invalid JSON, validation error, or invalid/expired state/code", body = ErrorResponse),
        (status = 409, description = "Conflict - Email already exists", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database, Redis, or OAuth provider error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn auth_google_login(
    user_agent: Option<TypedHeader<UserAgent>>,
    State(state): State<AppState>,
    Extension(anonymous): Extension<AnonymousUserContext>,
    ValidatedJson(payload): ValidatedJson<GoogleLoginRequest>,
) -> Result<Response, Errors> {
    let user_agent_str = extract_user_agent(user_agent);

    let result = service_google_sign_in(
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
