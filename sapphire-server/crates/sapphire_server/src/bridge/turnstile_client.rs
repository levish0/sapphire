use sapphire_errors::errors::Errors;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use tracing::error;

const TURNSTILE_VERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

#[derive(Debug, Deserialize)]
pub struct TurnstileResponse {
    pub success: bool,
    #[serde(rename = "error-codes", default)]
    pub error_codes: Vec<String>,
    #[serde(default)]
    pub challenge_ts: Option<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub cdata: Option<String>,
}

#[derive(Debug, Serialize)]
struct TurnstileRequest<'a> {
    secret: &'a str,
    response: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    remoteip: Option<&'a str>,
}

///
/// # Arguments
///
/// # Returns
pub async fn verify_turnstile_token(
    http_client: &HttpClient,
    secret_key: &str,
    token: &str,
    remote_ip: Option<&str>,
) -> Result<TurnstileResponse, Errors> {
    let request_body = TurnstileRequest {
        secret: secret_key,
        response: token,
        remoteip: remote_ip,
    };

    let response = http_client
        .post(TURNSTILE_VERIFY_URL)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            error!("Turnstile request failed: {e}");
            Errors::TurnstileServiceError
        })?;

    if !response.status().is_success() {
        return Err(Errors::TurnstileServiceError);
    }

    response.json::<TurnstileResponse>().await.map_err(|e| {
        error!("Turnstile response parse failed: {e}");
        Errors::TurnstileServiceError
    })
}
