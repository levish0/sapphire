use crate::bridge::worker_client;
use crate::repository::user::user_roles::repository_find_user_roles;
use crate::repository::user::{UserUpdateParams, repository_update_user};
use crate::service::auth::session_types::SessionContext;
use crate::state::WorkerClient;
use crate::utils::r2_url::build_r2_public_url;
use sapphire_dto::user::UserResponse;
use sapphire_dto::user::request::UpdateMyProfileRequest;
use sapphire_errors::errors::Errors;
use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;

///
///
/// - `repository_update_user`
/// - `repository_find_user_roles`
/// - `worker_client::index_user`
///
/// # Errors
pub async fn service_update_my_profile(
    db: &DatabaseConnection,
    worker: &WorkerClient,
    session: &SessionContext,
    request: UpdateMyProfileRequest,
) -> Result<UserResponse, Errors> {
    let params = UserUpdateParams {
        display_name: request.display_name,
        bio: request.bio.map(Some),
        ..Default::default()
    };

    let txn = db.begin().await?;

    let updated_user = repository_update_user(&txn, session.user_id, params).await?;
    let roles = repository_find_user_roles(&txn, session.user_id).await?;

    txn.commit().await?;

    info!(user_id = %session.user_id, "Profile updated");

    worker_client::index_user(worker, session.user_id)
        .await
        .ok();

    Ok(UserResponse {
        id: session.user_id.to_string(),
        email: updated_user.email,
        handle: updated_user.handle,
        display_name: updated_user.display_name,
        bio: updated_user.bio,
        profile_image: updated_user
            .profile_image
            .as_deref()
            .map(build_r2_public_url),
        banner_image: updated_user
            .banner_image
            .as_deref()
            .map(build_r2_public_url),
        roles,
        created_at: updated_user.created_at,
    })
}
