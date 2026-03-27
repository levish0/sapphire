use sapphire_entity::common::OAuthProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingSignupData {
    pub provider: OAuthProvider,
    pub provider_user_id: String,
    pub anonymous_user_id: String,
    pub email: String,
    pub display_name: String,
    pub profile_image: Option<String>,
}
