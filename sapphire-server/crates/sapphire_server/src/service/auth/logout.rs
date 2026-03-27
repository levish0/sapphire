use crate::service::auth::session::SessionService;
use redis::aio::ConnectionManager;
use sapphire_errors::errors::ServiceResult;
use tracing::info;

///
///
/// - `SessionService::delete_session`
///
/// # Errors
pub async fn service_logout(redis: &ConnectionManager, session_id: &str) -> ServiceResult<()> {
    SessionService::delete_session(redis, session_id).await?;

    info!(session_id = %session_id, "Logout");

    Ok(())
}
