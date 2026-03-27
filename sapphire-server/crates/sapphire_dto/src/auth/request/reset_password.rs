use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
/// Request payload for reset password request.
pub struct ResetPasswordRequest {
    #[validate(length(min = 1))]
    pub token: String,

    #[validate(length(
        min = 6,
        max = 20,
        message = "Password must be between 6 and 20 characters."
    ))]
    pub new_password: String,
}
