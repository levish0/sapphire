use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
/// Request payload for change password request.
pub struct ChangePasswordRequest {
    #[validate(length(min = 1))]
    pub current_password: String,

    #[validate(length(
        min = 6,
        max = 20,
        message = "Password must be between 6 and 20 characters."
    ))]
    pub new_password: String,
}
