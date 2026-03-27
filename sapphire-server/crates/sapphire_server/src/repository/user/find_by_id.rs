use sapphire_entity::users::{Entity as UserEntity, Model as UserModel};
use sapphire_errors::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

///
///
/// # Errors
pub async fn repository_find_user_by_id<C>(conn: &C, id: Uuid) -> Result<Option<UserModel>, Errors>
where
    C: ConnectionTrait,
{
    let user = UserEntity::find_by_id(id).one(conn).await?;

    Ok(user)
}
