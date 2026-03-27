use crate::extractors::RequiredSession;
use crate::service::user::profile::upload_banner_image::service_upload_banner_image;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::user::UploadUserImageRequest;
use sapphire_dto::user::UploadUserImageResponse;
use sapphire_dto::validator::multipart_validator::ValidatedMultipart;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/user/me/banner-image",
    request_body(content = UploadUserImageRequest, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Banner image uploaded successfully", body = UploadUserImageResponse),
        (status = 400, description = "Bad Request - Invalid image or validation error", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session", body = ErrorResponse),
        (status = 413, description = "Payload Too Large - Image exceeds size limit", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User",
)]
pub async fn upload_banner_image(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedMultipart(payload): ValidatedMultipart<UploadUserImageRequest>,
) -> Result<UploadUserImageResponse, Errors> {
    service_upload_banner_image(&state.db, &state.r2_assets, &session_context, payload).await
}
