use crate::connection::r2_assets_conn::R2AssetsClient;
use crate::repository::user::{
    UserUpdateParams, repository_get_user_by_id, repository_update_user,
};
use crate::service::auth::session_types::SessionContext;
use sapphire_errors::errors::Errors;
use sea_orm::DatabaseConnection;
use tracing::{info, warn};

///
///
/// - `repository_get_user_by_id`
/// - `repository_update_user`
/// - `R2AssetsClient::delete`
///
/// # Errors
pub async fn service_delete_banner_image(
    db: &DatabaseConnection,
    r2_assets: &R2AssetsClient,
    session: &SessionContext,
) -> Result<(), Errors> {
    let user = repository_get_user_by_id(db, session.user_id).await?;

    let Some(storage_key) = user.banner_image else {
        return Err(Errors::NotFound("No banner image to delete".to_string()));
    };

    // Delete from R2 (best effort)
    if let Err(e) = r2_assets.delete(&storage_key).await {
        warn!(user_id = %session.user_id, error = ?e, "Failed to delete banner image from R2, continuing with DB update");
    }

    repository_update_user(
        db,
        session.user_id,
        UserUpdateParams {
            banner_image: Some(None),
            ..Default::default()
        },
    )
    .await?;

    info!(user_id = %session.user_id, "Banner image deleted");

    Ok(())
}
