use reqwest::Client;
use std::time::Duration;
use tracing::{error, info};

const REQUEST_TIMEOUT_SECS: u64 = 30;
const CONNECT_TIMEOUT_SECS: u64 = 10;
const POOL_IDLE_TIMEOUT_SECS: u64 = 90;
const POOL_MAX_IDLE_PER_HOST: usize = 10;
const TCP_KEEPALIVE_SECS: u64 = 60;
const USER_AGENT: &str = "sapphire-server/1.0";

pub async fn create_http_client() -> Result<Client, reqwest::Error> {
    info!("Creating HTTP client");

    let client = Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
        .pool_idle_timeout(Duration::from_secs(POOL_IDLE_TIMEOUT_SECS))
        .pool_max_idle_per_host(POOL_MAX_IDLE_PER_HOST)
        .user_agent(USER_AGENT)
        .tcp_keepalive(Duration::from_secs(TCP_KEEPALIVE_SECS))
        .build()
        .map_err(|e| {
            error!("Failed to create HTTP client: {:?}", e);
            e
        })?;

    info!("Successfully created HTTP client");
    Ok(client)
}
