use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct TotpBackupCodesResponse {
    pub backup_codes: Vec<String>,
}

impl IntoResponse for TotpBackupCodesResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
