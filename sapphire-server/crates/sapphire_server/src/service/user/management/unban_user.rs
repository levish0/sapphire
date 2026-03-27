use crate::permission::PermissionService;
use crate::repository::moderation::repository_create_moderation_log;
use crate::repository::user::user_bans::{repository_delete_user_ban, repository_find_user_ban};
use crate::service::auth::session_types::SessionContext;
use sapphire_constants::ModerationAction;
use sapphire_dto::user::response::UnbanUserResponse;
use sapphire_entity::common::ModerationResourceType;
use sapphire_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;
use uuid::Uuid;

/// Remove an active ban from a user.
///
/// Rules:
/// - Only admins can unban users.
/// - Admin targets cannot be managed.
///
/// # Errors
/// - Returns `Errors::UserNotBanned` if the target has no active ban.
pub async fn service_unban_user(
    db: &DatabaseConnection,
    target_user_id: Uuid,
    reason: String,
    session: &SessionContext,
) -> ServiceResult<UnbanUserResponse> {
    PermissionService::require_admin_for_target(db, Some(session), target_user_id).await?;

    let txn = db.begin().await?;

    if repository_find_user_ban(&txn, target_user_id)
        .await?
        .is_none()
    {
        return Err(Errors::UserNotBanned);
    }

    repository_delete_user_ban(&txn, target_user_id).await?;

    repository_create_moderation_log(
        &txn,
        ModerationAction::UserUnban,
        Some(session.user_id),
        ModerationResourceType::User,
        Some(target_user_id),
        reason,
        None,
    )
    .await?;

    txn.commit().await?;

    info!(
        target_user_id = %target_user_id,
        actor_id = %session.user_id,
        "User unbanned"
    );

    Ok(UnbanUserResponse {
        user_id: target_user_id,
    })
}
