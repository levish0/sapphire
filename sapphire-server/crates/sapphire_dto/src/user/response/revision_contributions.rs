use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use sapphire_entity::common::DocumentNamespace;

/// Single revision contribution item
#[derive(Debug, Serialize, ToSchema)]
pub struct UserRevisionContributionItem {
    // Revision info
    pub revision_id: Uuid,
    pub revision_number: i32,
    pub edit_summary: String,
    pub content_length: i32,
    pub content_chars_added: i32,
    pub content_chars_removed: i32,
    pub hidden_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    // Document info
    pub document_id: Uuid,
    pub document_namespace: DocumentNamespace,
    pub document_title: String,
}

/// Response for user revision contributions
#[derive(Debug, Serialize, ToSchema)]
pub struct GetUserRevisionContributionsResponse {
    pub items: Vec<UserRevisionContributionItem>,
    /// Whether there are newer (more recent) contributions
    pub has_newer: bool,
    /// Whether there are older contributions
    pub has_older: bool,
}

impl IntoResponse for GetUserRevisionContributionsResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
