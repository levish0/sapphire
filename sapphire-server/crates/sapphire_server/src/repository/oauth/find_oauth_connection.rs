use sapphire_entity::common::OAuthProvider;
use sapphire_entity::user_oauth_connections::{
    Column as OAuthConnectionsColumn, Entity as OAuthConnectionsEntity,
    Model as OAuthConnectionModel,
};
use sapphire_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_oauth_connection<C>(
    conn: &C,
    user_id: Uuid,
    provider: OAuthProvider,
) -> Result<Option<OAuthConnectionModel>, Errors>
where
    C: ConnectionTrait,
{
    let connection = OAuthConnectionsEntity::find()
        .filter(OAuthConnectionsColumn::UserId.eq(user_id))
        .filter(OAuthConnectionsColumn::Provider.eq(provider))
        .one(conn)
        .await?;

    Ok(connection)
}
