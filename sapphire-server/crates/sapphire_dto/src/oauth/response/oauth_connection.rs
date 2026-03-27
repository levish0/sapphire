use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use sapphire_entity::common::OAuthProvider;
use sapphire_entity::user_oauth_connections::Model as OAuthConnectionModel;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct OAuthConnectionResponse {
    /// OAuth provider (Google, Github)
    pub provider: OAuthProvider,

    pub created_at: DateTime<Utc>,
}

impl From<OAuthConnectionModel> for OAuthConnectionResponse {
    fn from(model: OAuthConnectionModel) -> Self {
        Self {
            provider: model.provider,
            created_at: model.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OAuthConnectionListResponse {
    pub connections: Vec<OAuthConnectionResponse>,
}

impl IntoResponse for OAuthConnectionListResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
