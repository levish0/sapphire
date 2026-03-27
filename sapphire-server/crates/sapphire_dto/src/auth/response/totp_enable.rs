use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct TotpEnableResponse {
    pub backup_codes: Vec<String>,
}

impl IntoResponse for TotpEnableResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
