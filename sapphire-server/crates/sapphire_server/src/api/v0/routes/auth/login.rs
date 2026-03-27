use crate::service::auth::LoginResult;
use crate::service::auth::login::service_login;
use crate::state::AppState;
use crate::utils::extract::extract_user_agent::extract_user_agent;
use axum::response::IntoResponse;
use axum::{extract::State, response::Response};
use axum_extra::{TypedHeader, headers::UserAgent};
use sapphire_dto::auth::request::LoginRequest;
use sapphire_dto::auth::response::TotpRequiredResponse;
use sapphire_dto::auth::response::create_login_response;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 204, description = "Login successful"),
        (status = 202, description = "TOTP required", body = TotpRequiredResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Invalid credentials or password not set", body = ErrorResponse),
        (status = 404, description = "Not Found - User not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database or Redis error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn auth_login(
    user_agent: Option<TypedHeader<UserAgent>>,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<Response, Errors> {
    let user_agent = extract_user_agent(user_agent);
    let result = service_login(&state.db, &state.redis_session, payload, Some(user_agent)).await?;

    match result {
        LoginResult::SessionCreated {
            session_id,
            remember_me,
        } => create_login_response(session_id, remember_me),
        LoginResult::TotpRequired(temp_token) => {
            Ok(TotpRequiredResponse { temp_token }.into_response())
        }
    }
}
