use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
/// Request payload for confirm email change request.
pub struct ConfirmEmailChangeRequest {
    #[validate(length(min = 1))]
    pub token: String,
}
