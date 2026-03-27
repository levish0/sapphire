use crate::repository::user::repository_find_user_by_email;
use crate::service::auth::session::SessionService;
use crate::service::auth::totp::TotpTempToken;
use sapphire_dto::auth::request::LoginRequest;
use sapphire_errors::errors::{Errors, ServiceResult};
use tracing::info;

use crate::utils::crypto::password::verify_password;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;

pub enum LoginResult {
    SessionCreated {
        session_id: String,
        remember_me: bool,
    },
    TotpRequired(String),
}

///
///
/// - `repository_find_user_by_email`
/// - `verify_password`
/// - `TotpTempToken::create`
/// - `SessionService::create_session`
///
/// # Errors
pub async fn service_login(
    db: &DatabaseConnection,
    redis: &ConnectionManager,
    payload: LoginRequest,
    user_agent: Option<String>,
) -> ServiceResult<LoginResult> {
    let user = repository_find_user_by_email(db, payload.email.clone())
        .await?
        .ok_or(Errors::InvalidCredentials)?;

    let password_hash = user.password.ok_or(Errors::InvalidCredentials)?;
    verify_password(&payload.password, &password_hash).map_err(|_| Errors::InvalidCredentials)?;

    if user.totp_enabled_at.is_some() {
        let temp_token =
            TotpTempToken::create(redis, user.id, user_agent, payload.remember_me).await?;

        info!(user_id = %user.id, "Login requires TOTP");
        return Ok(LoginResult::TotpRequired(temp_token.token));
    }

    let session = SessionService::create_session(redis, user.id.to_string(), user_agent).await?;

    info!(user_id = %user.id, "Login successful");

    Ok(LoginResult::SessionCreated {
        session_id: session.session_id,
        remember_me: payload.remember_me,
    })
}
