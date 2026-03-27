use crate::extractors::RequiredSession;
use crate::service::user::profile::delete_banner_image::service_delete_banner_image;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    delete,
    path = "/v0/user/me/banner-image",
    responses(
        (status = 204, description = "Banner image deleted successfully"),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 404, description = "Not Found - No banner image to delete", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User",
)]
pub async fn delete_banner_image(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
) -> Result<StatusCode, Errors> {
    service_delete_banner_image(&state.db, &state.r2_assets, &session_context).await?;
    Ok(StatusCode::NO_CONTENT)
}
