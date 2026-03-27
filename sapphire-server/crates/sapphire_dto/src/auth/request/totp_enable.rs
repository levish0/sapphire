use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TotpEnableRequest {
    #[validate(length(equal = 6, message = "TOTP code must be 6 digits"))]
    pub code: String,
}
