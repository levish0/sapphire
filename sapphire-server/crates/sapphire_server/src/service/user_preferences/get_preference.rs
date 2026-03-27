use crate::repository::user_preferences::repository_find_user_preference_by_user_id_and_key;
use crate::service::auth::session_types::SessionContext;
use sapphire_constants::UserPreferenceKey;
use sapphire_dto::user_preferences::UserPreferenceResponse;
use sapphire_errors::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;
use std::str::FromStr;

///
///
/// - `repository_find_user_preference_by_user_id_and_key`
///
/// # Errors
pub async fn service_get_user_preference(
    db: &DatabaseConnection,
    session: &SessionContext,
    key: UserPreferenceKey,
) -> ServiceResult<UserPreferenceResponse> {
    let preference =
        repository_find_user_preference_by_user_id_and_key(db, session.user_id, key).await?;

    match preference {
        Some(pref) => {
            let key = UserPreferenceKey::from_str(&pref.key).map_err(Errors::SysInternalError)?;
            Ok(UserPreferenceResponse {
                key,
                value: pref.value,
                updated_at: pref.updated_at,
            })
        }
        None => Err(Errors::NotFound("User preference not found".to_string())),
    }
}
