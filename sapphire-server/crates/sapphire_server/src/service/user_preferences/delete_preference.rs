use crate::repository::user_preferences::repository_delete_user_preference;
use crate::service::auth::session_types::SessionContext;
use sapphire_constants::UserPreferenceKey;
use sapphire_errors::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;

///
///
/// - `repository_delete_user_preference`
///
/// # Errors
pub async fn service_delete_user_preference(
    db: &DatabaseConnection,
    session: &SessionContext,
    key: UserPreferenceKey,
) -> ServiceResult<()> {
    let deleted = repository_delete_user_preference(db, session.user_id, key).await?;

    if deleted {
        Ok(())
    } else {
        Err(Errors::NotFound("User preference not found".to_string()))
    }
}
