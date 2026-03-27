use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct TotpStatusResponse {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_codes_remaining: Option<usize>,
}

impl IntoResponse for TotpStatusResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
