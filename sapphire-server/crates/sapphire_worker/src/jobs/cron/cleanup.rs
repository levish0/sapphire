use sea_orm::DatabaseConnection;

use super::cleanup_expired_bans::run_cleanup_expired_bans;
use super::cleanup_expired_roles::run_cleanup_expired_roles;

/// Batch size for cleanup operations
const BATCH_SIZE: u64 = 1000;

/// Run the cleanup job
///
/// Cleans up:
/// - Expired user bans (expires_at < NOW())
/// - Expired user roles (expires_at < NOW())
pub async fn run_cleanup(db: &DatabaseConnection) {
    tracing::info!("Starting scheduled cleanup job");

    // 1. Cleanup expired bans
    let bans = match run_cleanup_expired_bans(db, BATCH_SIZE).await {
        Ok((count, _user_ids)) => count,
        Err(e) => {
            tracing::error!(error = %e, "Failed to cleanup expired user bans");
            0
        }
    };

    // 2. Cleanup expired roles
    let roles = match run_cleanup_expired_roles(db, BATCH_SIZE).await {
        Ok((count, _user_ids)) => count,
        Err(e) => {
            tracing::error!(error = %e, "Failed to cleanup expired user roles");
            0
        }
    };

    tracing::info!(
        expired_bans = bans,
        expired_roles = roles,
        "Cleanup job completed"
    );
}
