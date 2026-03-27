use crate::repository::user_preferences::repository_find_user_preferences_by_user_id;
use crate::service::auth::session_types::SessionContext;
use sapphire_constants::UserPreferenceKey;
use sapphire_dto::user_preferences::{UserPreferenceResponse, UserPreferencesResponse};
use sapphire_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use std::str::FromStr;

///
///
/// - `repository_find_user_preferences_by_user_id`
///
/// # Errors
pub async fn service_get_user_preferences(
    db: &DatabaseConnection,
    session: &SessionContext,
) -> ServiceResult<UserPreferencesResponse> {
    let preferences = repository_find_user_preferences_by_user_id(db, session.user_id).await?;

    // Filter out unknown keys (legacy data)
    let preference_responses: Vec<UserPreferenceResponse> = preferences
        .into_iter()
        .filter_map(|pref| {
            UserPreferenceKey::from_str(&pref.key)
                .ok()
                .map(|key| UserPreferenceResponse {
                    key,
                    value: pref.value,
                    updated_at: pref.updated_at,
                })
        })
        .collect();

    Ok(UserPreferencesResponse {
        preferences: preference_responses,
    })
}
