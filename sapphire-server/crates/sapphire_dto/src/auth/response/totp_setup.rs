use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct TotpSetupResponse {
    pub qr_code_base64: String,
    pub qr_code_uri: String,
}

impl IntoResponse for TotpSetupResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
