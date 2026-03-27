use crate::service::user::account::check_handle_available::service_check_handle_available;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::user::{CheckHandleAvailablePath, CheckHandleAvailableResponse};
use sapphire_dto::validator::path_validator::ValidatedPath;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    get,
    path = "/v0/users/handle/{handle}/available",
    params(CheckHandleAvailablePath),
    responses(
        (status = 200, description = "Handle availability checked", body = CheckHandleAvailableResponse),
        (status = 400, description = "Bad request - Invalid handle format", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    tag = "User"
)]
pub async fn check_handle_available(
    State(state): State<AppState>,
    ValidatedPath(path): ValidatedPath<CheckHandleAvailablePath>,
) -> Result<CheckHandleAvailableResponse, Errors> {
    service_check_handle_available(&state.db, &path.handle).await
}
