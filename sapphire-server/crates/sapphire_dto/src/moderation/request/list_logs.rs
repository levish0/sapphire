use crate::pagination::CursorDirection;
use chrono::{DateTime, Utc};
use sapphire_constants::ModerationAction;
use sapphire_entity::common::ModerationResourceType;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
/// Request payload for list moderation logs request.
pub struct ListModerationLogsRequest {
    pub cursor_id: Option<Uuid>,
    pub cursor_direction: Option<CursorDirection>,
    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100."))]
    pub limit: u64,
    pub actor_id: Option<Uuid>,
    pub resource_type: Option<ModerationResourceType>,
    pub resource_id: Option<Uuid>,
    pub actions: Option<Vec<ModerationAction>>,
    /// Inclusive lower bound for log creation time (UTC).
    pub created_from: Option<DateTime<Utc>>,
    /// Exclusive upper bound for log creation time (UTC).
    pub created_to: Option<DateTime<Utc>>,
}
