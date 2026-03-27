use sapphire_entity::common::OAuthProvider;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct UnlinkOAuthRequest {
    /// OAuth provider to unlink (Google or Github)
    pub provider: OAuthProvider,
}
