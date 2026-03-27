use crate::pagination::CursorDirection;
use crate::validator::string_validator::validate_not_blank;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate, IntoParams)]
#[into_params(parameter_in = Query)]
/// Request payload for get user revision contributions request.
pub struct GetUserRevisionContributionsRequest {
    /// User handle (username)
    #[validate(length(
        min = 1,
        max = 32,
        message = "Handle must be between 1 and 32 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub handle: String,
    /// Cursor revision ID for pagination. None means get latest revisions.
    pub cursor: Option<Uuid>,
    /// Cursor direction for pagination (Older/Newer). Defaults to Older.
    pub direction: Option<CursorDirection>,
    /// Number of items per page (max: 100)
    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100."))]
    pub limit: u32,
}
