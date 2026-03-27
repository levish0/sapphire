use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
/// Request payload for forgot password request.
pub struct ForgotPasswordRequest {
    #[validate(email)]
    pub email: String,
}
