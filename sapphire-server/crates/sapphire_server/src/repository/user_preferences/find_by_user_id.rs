use sapphire_entity::user_preferences::{
    Column as UserPreferenceColumn, Entity as UserPreferenceEntity, Model as UserPreferenceModel,
};
use sapphire_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

///
///
/// - `service_get_user_preferences`
///
/// # Errors
pub async fn repository_find_user_preferences_by_user_id<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<Vec<UserPreferenceModel>, Errors>
where
    C: ConnectionTrait,
{
    let preferences = UserPreferenceEntity::find()
        .filter(UserPreferenceColumn::UserId.eq(user_id))
        .all(conn)
        .await?;

    Ok(preferences)
}
