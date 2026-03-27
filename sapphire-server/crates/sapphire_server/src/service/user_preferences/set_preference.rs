use crate::repository::user_preferences::repository_upsert_user_preference;
use crate::service::auth::session_types::SessionContext;
use sapphire_constants::UserPreferenceKey;
use sapphire_dto::user_preferences::UserPreferenceResponse;
use sapphire_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, JsonValue};
use std::str::FromStr;

///
///
/// - `repository_upsert_user_preference`
///
/// # Errors
pub async fn service_set_user_preference(
    db: &DatabaseConnection,
    session: &SessionContext,
    key: UserPreferenceKey,
    value: JsonValue,
) -> ServiceResult<UserPreferenceResponse> {
    let preference = repository_upsert_user_preference(db, session.user_id, key, value).await?;

    let key = UserPreferenceKey::from_str(&preference.key).map_err(Errors::SysInternalError)?;
    Ok(UserPreferenceResponse {
        key,
        value: preference.value,
        updated_at: preference.updated_at,
    })
}
