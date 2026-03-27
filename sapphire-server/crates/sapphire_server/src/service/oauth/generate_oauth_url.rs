use crate::service::oauth::provider::client::generate_auth_url;
use crate::service::oauth::provider::config::OAuthProviderConfig;
use crate::service::oauth::types::OAuthStateData;
use crate::utils::redis_cache::store_json_for_token_with_ttl;
use redis::aio::ConnectionManager;
use sapphire_dto::oauth::request::OAuthAuthorizeFlow;
use sapphire_dto::oauth::response::OAuthUrlResponse;
use sapphire_entity::common::OAuthProvider;
use sapphire_errors::errors::ServiceResult;
use uuid::Uuid;

pub async fn service_generate_oauth_url<P: OAuthProviderConfig>(
    redis_conn: &ConnectionManager,
    anonymous_user_id: &str,
    flow: OAuthAuthorizeFlow,
    provider: OAuthProvider,
) -> ServiceResult<OAuthUrlResponse> {
    let state = Uuid::now_v7().to_string();

    let (auth_url, _state, pkce_verifier) = generate_auth_url::<P>(state.clone())?;

    let state_data = OAuthStateData {
        pkce_verifier,
        flow,
        provider,
        anonymous_user_id: anonymous_user_id.to_string(),
    };
    store_json_for_token_with_ttl(
        redis_conn,
        &state,
        sapphire_constants::oauth_state_key,
        &state_data,
        sapphire_constants::OAUTH_STATE_TTL_SECONDS,
    )
    .await?;

    Ok(OAuthUrlResponse { auth_url })
}
