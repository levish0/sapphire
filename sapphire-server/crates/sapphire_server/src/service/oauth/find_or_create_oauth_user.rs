use crate::repository::oauth::create_oauth_connection::repository_create_oauth_connection;
use crate::repository::oauth::create_oauth_user::repository_create_oauth_user;
use crate::repository::oauth::find_user_by_oauth::repository_find_user_by_oauth;
use crate::repository::user::find_by_email::repository_find_user_by_email;
use crate::repository::user::find_by_handle::repository_find_user_by_handle;
use crate::service::auth::verify_email::{
    find_pending_email_signup_by_email, find_pending_email_signup_by_handle,
};
use crate::service::oauth::types::OAuthUserResult;
use sapphire_entity::common::OAuthProvider;
use sapphire_errors::errors::{Errors, ServiceResult};
use redis::aio::ConnectionManager;
use sea_orm::ConnectionTrait;
use tracing::info;

///
/// # Arguments
///
/// # Returns
pub async fn service_find_or_create_oauth_user<C>(
    conn: &C,
    redis_conn: &ConnectionManager,
    provider: OAuthProvider,
    provider_user_id: &str,
    email: &str,
    display_name: &str,
    handle: Option<&str>,
    profile_image: Option<String>,
) -> ServiceResult<OAuthUserResult>
where
    C: ConnectionTrait,
{
    if let Some(user) =
        repository_find_user_by_oauth(conn, provider.clone(), provider_user_id).await?
    {
        return Ok(OAuthUserResult {
            user,
            is_new_user: false,
        });
    }

    if repository_find_user_by_email(conn, email.to_string())
        .await?
        .is_some()
    {
        return Err(Errors::OauthEmailAlreadyExists);
    }

    // Also reject if a pending email/password signup already holds this email.
    if find_pending_email_signup_by_email(redis_conn, email)
        .await?
        .is_some()
    {
        return Err(Errors::OauthEmailAlreadyExists);
    }

    let handle = handle.ok_or(Errors::OauthHandleRequired)?;

    if repository_find_user_by_handle(conn, handle.to_string())
        .await?
        .is_some()
    {
        return Err(Errors::UserHandleAlreadyExists);
    }

    // Also reject if a pending email/password signup already holds this handle.
    if find_pending_email_signup_by_handle(redis_conn, handle)
        .await?
        .is_some()
    {
        return Err(Errors::UserHandleAlreadyExists);
    }

    let new_user =
        repository_create_oauth_user(conn, email, display_name, handle, profile_image).await?;

    repository_create_oauth_connection(conn, &new_user.id, provider.clone(), provider_user_id)
        .await?;

    info!(user_id = %new_user.id, provider = ?provider, "OAuth user created");

    Ok(OAuthUserResult {
        user: new_user,
        is_new_user: true,
    })
}
