use crate::bridge::worker_client;
use crate::connection::r2_assets_conn::R2AssetsClient;
use crate::repository::user::{UserUpdateParams, repository_update_user};
use crate::service::auth::session_types::SessionContext;
use crate::state::WorkerClient;
use crate::utils::image_processor::image_validator::{
    MAX_IMAGE_SIZE, generate_image_hash, process_image_for_upload, validate_image,
};
use sapphire_constants::user_image_key;
use sapphire_dto::user::UploadUserImageRequest;
use sapphire_dto::user::UploadUserImageResponse;
use sapphire_errors::errors::Errors;
use sea_orm::DatabaseConnection;
use tracing::{info, warn};

///
///
/// - `validate_image` / `process_image_for_upload`
/// - `R2AssetsClient::upload_with_content_type`
/// - `repository_update_user`
/// - `worker_client::index_user`
///
/// # Errors
pub async fn service_upload_profile_image(
    db: &DatabaseConnection,
    r2_assets: &R2AssetsClient,
    worker: &WorkerClient,
    session: &SessionContext,
    payload: UploadUserImageRequest,
) -> Result<UploadUserImageResponse, Errors> {
    let image_info = validate_image(&payload.file, MAX_IMAGE_SIZE)?;

    let processed = process_image_for_upload(&payload.file, &image_info.mime_type)?;

    let hash = generate_image_hash(&processed.data);
    let storage_key = user_image_key(&hash, &processed.extension);

    r2_assets
        .upload_with_content_type(&storage_key, processed.data, &processed.content_type)
        .await
        .map_err(|e| {
            warn!(user_id = %session.user_id, error = ?e, "Failed to upload profile image to R2");
            Errors::SysInternalError("Failed to upload image to storage".to_string())
        })?;

    repository_update_user(
        db,
        session.user_id,
        UserUpdateParams {
            profile_image: Some(Some(storage_key.clone())),
            ..Default::default()
        },
    )
    .await?;

    let image_url = r2_assets.get_public_url(&storage_key);

    worker_client::index_user(worker, session.user_id)
        .await
        .ok();

    info!(user_id = %session.user_id, storage_key = %storage_key, "Profile image uploaded");

    Ok(UploadUserImageResponse { image_url })
}
