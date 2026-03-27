use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
/// Request payload for pending signup verification email resend.
pub struct ResendVerificationEmailRequest {
    /// Verification email recipient address
    #[validate(email)]
    pub email: String,
}
