use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;

use crate::bridge::turnstile_client::verify_turnstile_token;
use crate::state::AppState;
use sapphire_config::ServerConfig;
use sapphire_errors::errors::Errors;

pub const TURNSTILE_TOKEN_HEADER: &str = "X-Turnstile-Token";

///
///
/// ```rust,ignore
/// pub async fn create_document(
///     State(state): State<AppState>,
///     Json(req): Json<CreateDocumentRequest>,
/// ) -> Result<impl IntoResponse, Errors> {
/// }
/// ```
///
/// ```typescript
/// const response = await fetch('/api/v0/document', {
///   method: 'POST',
///   headers: {
///     'Content-Type': 'application/json',
///     'X-Turnstile-Token': turnstileToken,
///   },
///   body: JSON.stringify(data),
/// });
/// ```
#[derive(Debug, Clone)]
pub struct TurnstileVerified;

impl<S> FromRequestParts<S> for TurnstileVerified
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = Errors;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let token = parts
            .headers
            .get(TURNSTILE_TOKEN_HEADER)
            .and_then(|v| v.to_str().ok())
            .ok_or(Errors::TurnstileTokenMissing)?;

        let remote_ip = parts
            .headers
            .get("CF-Connecting-IP")
            .and_then(|v| v.to_str().ok());

        let config = ServerConfig::get();
        let response = verify_turnstile_token(
            &app_state.http_client,
            &config.turnstile_secret_key,
            token,
            remote_ip,
        )
        .await?;

        if !response.success {
            return Err(Errors::TurnstileVerificationFailed);
        }

        Ok(TurnstileVerified)
    }
}
