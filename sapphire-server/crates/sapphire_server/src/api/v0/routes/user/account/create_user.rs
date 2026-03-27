use crate::service::user::account::create_user::service_create_user;
use crate::state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sapphire_dto::user::{CreateUserRequest, CreateUserResponse};
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/users",
    request_body = CreateUserRequest,
    responses(
        (status = 202, description = "Verification email sent. Complete signup from email", body = CreateUserResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error", body = ErrorResponse),
        (status = 409, description = "Conflict - User with this email or handle already exists", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database or Redis error", body = ErrorResponse),
        (status = 502, description = "Bad Gateway - Worker service request failed or returned invalid response", body = ErrorResponse),
        (status = 503, description = "Service Unavailable - Worker service connection failed", body = ErrorResponse),
    ),
    tag = "User"
)]
pub async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response =
        service_create_user(&state.db, &state.redis_session, &state.worker, payload).await?;

    Ok((StatusCode::ACCEPTED, Json(response)))
}
