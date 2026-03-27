use sapphire_constants::UserPreferenceKey;
use sapphire_entity::user_preferences::{
    Column as UserPreferenceColumn, Entity as UserPreferenceEntity,
};
use sapphire_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

///
///
/// - `service_delete_user_preference`
///
/// # Errors
pub async fn repository_delete_user_preference<C>(
    conn: &C,
    user_id: Uuid,
    key: UserPreferenceKey,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let result = UserPreferenceEntity::delete_many()
        .filter(UserPreferenceColumn::UserId.eq(user_id))
        .filter(UserPreferenceColumn::Key.eq(key.as_str()))
        .exec(conn)
        .await?;

    Ok(result.rows_affected > 0)
}
