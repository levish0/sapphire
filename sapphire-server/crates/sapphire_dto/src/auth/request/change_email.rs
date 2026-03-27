use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
/// Request payload for change email request.
pub struct ChangeEmailRequest {
    #[validate(length(min = 1))]
    pub password: String,

    #[validate(email(message = "Invalid email format."))]
    pub new_email: String,
}
