use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use sapphire_constants::UserPreferenceKey;
use sea_orm::JsonValue;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
/// Response payload for user preference response.
pub struct UserPreferenceResponse {
    pub key: UserPreferenceKey,
    pub value: JsonValue,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for UserPreferenceResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize, ToSchema)]
/// Response payload for user preferences response.
pub struct UserPreferencesResponse {
    pub preferences: Vec<UserPreferenceResponse>,
}

impl IntoResponse for UserPreferencesResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
