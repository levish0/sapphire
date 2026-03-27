use chrono::Utc;
use sapphire_constants::UserPreferenceKey;
use sapphire_entity::user_preferences::{
    ActiveModel as UserPreferenceActiveModel, Column as UserPreferenceColumn,
    Entity as UserPreferenceEntity, Model as UserPreferenceModel,
};
use sapphire_errors::errors::Errors;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ConnectionTrait, EntityTrait, JsonValue, Set};
use uuid::Uuid;

///
///
/// - `service_set_user_preferences_bulk`
///
/// # Errors
pub async fn repository_upsert_user_preferences_bulk<C>(
    conn: &C,
    user_id: Uuid,
    preferences: Vec<(UserPreferenceKey, JsonValue)>,
) -> Result<Vec<UserPreferenceModel>, Errors>
where
    C: ConnectionTrait,
{
    if preferences.is_empty() {
        return Ok(vec![]);
    }

    let now = Utc::now();
    let active_models: Vec<UserPreferenceActiveModel> = preferences
        .into_iter()
        .map(|(key, value)| UserPreferenceActiveModel {
            user_id: Set(user_id),
            key: Set(key.to_string()),
            value: Set(value),
            updated_at: Set(now),
            ..Default::default()
        })
        .collect();

    let results = UserPreferenceEntity::insert_many(active_models)
        .on_conflict(
            OnConflict::columns([UserPreferenceColumn::UserId, UserPreferenceColumn::Key])
                .update_columns([UserPreferenceColumn::Value, UserPreferenceColumn::UpdatedAt])
                .to_owned(),
        )
        .exec_with_returning(conn)
        .await?;

    Ok(results)
}
