use crate::bridge::worker_client;
use crate::connection::r2_assets_conn::R2AssetsClient;
use crate::repository::user::{
    UserUpdateParams, repository_get_user_by_id, repository_update_user,
};
use crate::service::auth::session_types::SessionContext;
use crate::state::WorkerClient;
use sapphire_errors::errors::Errors;
use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::{info, warn};

///
///
/// - `repository_get_user_by_id`
/// - `repository_update_user`
/// - `worker_client::index_user`
///
/// # Errors
pub async fn service_delete_profile_image(
    db: &DatabaseConnection,
    r2_assets: &R2AssetsClient,
    worker: &WorkerClient,
    session: &SessionContext,
) -> Result<(), Errors> {
    let txn = db.begin().await?;

    let user = repository_get_user_by_id(&txn, session.user_id).await?;

    let Some(storage_key) = user.profile_image else {
        return Err(Errors::NotFound("No profile image to delete".to_string()));
    };

    repository_update_user(
        &txn,
        session.user_id,
        UserUpdateParams {
            profile_image: Some(None),
            ..Default::default()
        },
    )
    .await?;

    txn.commit().await?;

    if let Err(e) = r2_assets.delete(&storage_key).await {
        warn!(user_id = %session.user_id, error = ?e, "Failed to delete profile image from R2, DB already updated");
    }

    worker_client::index_user(worker, session.user_id)
        .await
        .ok();

    info!(user_id = %session.user_id, "Profile image deleted");

    Ok(())
}
