use sapphire_constants::UserPreferenceKey;
use sea_orm::JsonValue;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
/// Request payload for set user preference request.
pub struct SetUserPreferenceRequest {
    pub value: JsonValue,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
/// Request payload for preference item.
pub struct PreferenceItem {
    pub key: UserPreferenceKey,
    pub value: JsonValue,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
/// Request payload for set user preferences bulk request.
pub struct SetUserPreferencesBulkRequest {
    #[validate(length(min = 1, message = "At least one preference is required."))]
    pub preferences: Vec<PreferenceItem>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
#[into_params(parameter_in = Path)]
/// Request payload for user preference key path.
pub struct UserPreferenceKeyPath {
    pub key: UserPreferenceKey,
}
