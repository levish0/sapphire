use crate::permission::PermissionService;
use crate::repository::moderation::repository_create_moderation_log;
use crate::repository::user::user_roles::{
    repository_create_user_role, repository_delete_expired_user_role, repository_find_user_roles,
};
use crate::service::auth::session_types::SessionContext;
use chrono::{DateTime, Utc};
use sapphire_constants::ModerationAction;
use sapphire_dto::user::response::GrantRoleResponse;
use sapphire_entity::common::{ModerationResourceType, Role};
use sapphire_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

/// Grant an explicit role to a user.
///
/// Rules:
/// - Only admins can grant roles.
/// - Admin targets cannot be managed.
///
/// # Errors
/// - Returns `Errors::UserAlreadyHasRole` if the target already holds the role.
pub async fn service_grant_role(
    db: &DatabaseConnection,
    target_user_id: Uuid,
    role: Role,
    expires_at: Option<DateTime<Utc>>,
    reason: String,
    session: &SessionContext,
) -> ServiceResult<GrantRoleResponse> {
    PermissionService::require_admin_for_target(db, Some(session), target_user_id).await?;

    let txn = db.begin().await?;

    // Ensure the target does not already hold the role.
    let existing_roles = repository_find_user_roles(&txn, target_user_id).await?;
    if existing_roles.contains(&role) {
        return Err(Errors::UserAlreadyHasRole);
    }

    // Remove expired leftovers first to avoid unique-key conflicts on re-grant.
    repository_delete_expired_user_role(&txn, target_user_id, role).await?;

    // Insert the new role row.
    let user_role = repository_create_user_role(&txn, target_user_id, role, expires_at).await?;

    // Record the moderation event.
    repository_create_moderation_log(
        &txn,
        ModerationAction::UserGrantRole,
        Some(session.user_id),
        ModerationResourceType::User,
        Some(target_user_id),
        reason,
        Some(json!({
            "role": role.as_str(),
            "expires_at": expires_at
        })),
    )
    .await?;

    txn.commit().await?;

    info!(
        target_user_id = %target_user_id,
        role = %role,
        actor_id = %session.user_id,
        "Role granted"
    );

    Ok(GrantRoleResponse {
        user_id: target_user_id,
        role: user_role.role,
        expires_at: user_role.expires_at,
    })
}
