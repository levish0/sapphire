use crate::permission::PermissionService;
use crate::repository::moderation::repository_create_moderation_log;
use crate::repository::user::user_roles::{
    repository_delete_user_role, repository_find_user_roles,
};
use crate::service::auth::session_types::SessionContext;
use sapphire_constants::ModerationAction;
use sapphire_dto::user::response::RevokeRoleResponse;
use sapphire_entity::common::{ModerationResourceType, Role};
use sapphire_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

/// Revoke an explicit role from a user.
///
/// Rules:
/// - Only admins can revoke roles.
/// - Admin targets cannot be managed.
///
/// # Errors
/// - Returns `Errors::UserDoesNotHaveRole` if the target does not hold the role.
pub async fn service_revoke_role(
    db: &DatabaseConnection,
    target_user_id: Uuid,
    role: Role,
    reason: String,
    session: &SessionContext,
) -> ServiceResult<RevokeRoleResponse> {
    PermissionService::require_admin_for_target(db, Some(session), target_user_id).await?;

    let txn = db.begin().await?;

    // Ensure the role is currently active before deleting it.
    let active_roles = repository_find_user_roles(&txn, target_user_id).await?;
    if !active_roles.contains(&role) {
        return Err(Errors::UserDoesNotHaveRole);
    }

    // Delete the role row, including expired leftovers for the same role.
    repository_delete_user_role(&txn, target_user_id, role).await?;

    // Record the moderation event.
    repository_create_moderation_log(
        &txn,
        ModerationAction::UserRevokeRole,
        Some(session.user_id),
        ModerationResourceType::User,
        Some(target_user_id),
        reason,
        Some(json!({
            "role": role.as_str()
        })),
    )
    .await?;

    txn.commit().await?;

    info!(
        target_user_id = %target_user_id,
        role = %role,
        actor_id = %session.user_id,
        "Role revoked"
    );

    Ok(RevokeRoleResponse {
        user_id: target_user_id,
        role,
    })
}
