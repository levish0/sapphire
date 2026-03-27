use crate::repository::user_preferences::repository_upsert_user_preferences_bulk;
use crate::service::auth::session_types::SessionContext;
use sapphire_constants::UserPreferenceKey;
use sapphire_dto::user_preferences::{
    PreferenceItem, UserPreferenceResponse, UserPreferencesResponse,
};
use sapphire_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use std::str::FromStr;

///
///
/// - `repository_upsert_user_preferences_bulk`
///
/// # Errors
pub async fn service_set_user_preferences_bulk(
    db: &DatabaseConnection,
    session: &SessionContext,
    preferences: Vec<PreferenceItem>,
) -> ServiceResult<UserPreferencesResponse> {
    let preference_tuples: Vec<_> = preferences.into_iter().map(|p| (p.key, p.value)).collect();

    let results =
        repository_upsert_user_preferences_bulk(db, session.user_id, preference_tuples).await?;

    let preference_responses: Vec<UserPreferenceResponse> = results
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
