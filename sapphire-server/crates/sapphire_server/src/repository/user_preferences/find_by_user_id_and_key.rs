use sapphire_constants::UserPreferenceKey;
use sapphire_entity::user_preferences::{
    Column as UserPreferenceColumn, Entity as UserPreferenceEntity, Model as UserPreferenceModel,
};
use sapphire_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

///
///
/// - `service_get_user_preference`
///
/// # Errors
pub async fn repository_find_user_preference_by_user_id_and_key<C>(
    conn: &C,
    user_id: Uuid,
    key: UserPreferenceKey,
) -> Result<Option<UserPreferenceModel>, Errors>
where
    C: ConnectionTrait,
{
    let preference = UserPreferenceEntity::find()
        .filter(UserPreferenceColumn::UserId.eq(user_id))
        .filter(UserPreferenceColumn::Key.eq(key.as_str()))
        .one(conn)
        .await?;

    Ok(preference)
}
