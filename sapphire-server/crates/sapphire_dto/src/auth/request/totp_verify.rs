use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TotpVerifyRequest {
    pub temp_token: String,
    #[validate(length(min = 6, max = 8, message = "Code must be 6-8 characters"))]
    pub code: String,
}
