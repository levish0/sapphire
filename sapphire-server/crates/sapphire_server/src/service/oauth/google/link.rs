use super::{GoogleProvider, fetch_google_user_info};
use crate::repository::oauth::create_oauth_connection::repository_create_oauth_connection;
use crate::repository::oauth::find_oauth_connection::repository_find_oauth_connection;
use crate::repository::oauth::find_user_by_oauth::repository_find_user_by_oauth;
use crate::service::oauth::provider::client::exchange_code;
use crate::service::oauth::types::OAuthStateData;
use crate::utils::redis_cache::get_json_and_delete;
use sapphire_constants::oauth_state_key;
use sapphire_dto::oauth::request::OAuthAuthorizeFlow;
use sapphire_entity::common::OAuthProvider;
use sapphire_errors::errors::{Errors, ServiceResult};
use redis::aio::ConnectionManager;
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_link_google_oauth(
    db: &DatabaseConnection,
    redis_conn: &ConnectionManager,
    http_client: &reqwest::Client,
    user_id: Uuid,
    code: &str,
    state: &str,
    anonymous_user_id: &str,
) -> ServiceResult<()> {
    let state_key = oauth_state_key(state);
    let state_data: OAuthStateData = get_json_and_delete(
        redis_conn,
        &state_key,
        || Errors::OauthInvalidState,
        |_| Errors::OauthInvalidState,
    )
    .await?;

    if state_data.provider != OAuthProvider::Google
        || state_data.flow != OAuthAuthorizeFlow::Link
        || state_data.anonymous_user_id != anonymous_user_id
    {
        return Err(Errors::OauthInvalidState);
    }

    let access_token =
        exchange_code::<GoogleProvider>(http_client, code, &state_data.pkce_verifier).await?;

    let user_info = fetch_google_user_info(http_client, &access_token).await?;

    if !user_info.verified_email {
        return Err(Errors::OauthEmailNotVerified);
    }

    let txn = db.begin().await?;

    if repository_find_user_by_oauth(&txn, OAuthProvider::Google, &user_info.id)
        .await?
        .is_some()
    {
        return Err(Errors::OauthAccountAlreadyLinked);
    }

    if repository_find_oauth_connection(&txn, user_id, OAuthProvider::Google)
        .await?
        .is_some()
    {
        return Err(Errors::OauthAccountAlreadyLinked);
    }

    repository_create_oauth_connection(&txn, &user_id, OAuthProvider::Google, &user_info.id)
        .await?;

    txn.commit().await?;

    Ok(())
}
