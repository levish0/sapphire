use crate::validator::string_validator::validate_not_blank;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CompleteSignupRequest {
    #[validate(length(min = 1, message = "Pending token is required"))]
    pub pending_token: String,

    #[validate(length(
        min = 3,
        max = 20,
        message = "Handle must be between 3 and 20 characters"
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub handle: String,
}
