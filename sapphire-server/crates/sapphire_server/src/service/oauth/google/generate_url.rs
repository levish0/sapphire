use super::GoogleProvider;
use crate::service::oauth::generate_oauth_url::service_generate_oauth_url;
use redis::aio::ConnectionManager;
use sapphire_dto::oauth::request::OAuthAuthorizeFlow;
use sapphire_dto::oauth::response::OAuthUrlResponse;
use sapphire_entity::common::OAuthProvider;
use sapphire_errors::errors::ServiceResult;

pub async fn service_generate_google_oauth_url(
    redis_conn: &ConnectionManager,
    anonymous_user_id: &str,
    flow: OAuthAuthorizeFlow,
) -> ServiceResult<OAuthUrlResponse> {
    service_generate_oauth_url::<GoogleProvider>(
        redis_conn,
        anonymous_user_id,
        flow,
        OAuthProvider::Google,
    )
    .await
}
