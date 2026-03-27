use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
/// Response payload for start reindex response.
pub struct StartReindexResponse {
    pub reindex_id: Uuid,
    pub entity_type: String,
    pub message: String,
}

impl IntoResponse for StartReindexResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::ACCEPTED, Json(self)).into_response()
    }
}
