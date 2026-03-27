use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
/// Request enum for o auth authorize flow.
pub enum OAuthAuthorizeFlow {
    Login,
    Link,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
/// Request payload for o auth authorize query.
pub struct OAuthAuthorizeQuery {
    #[serde(default)]
    pub flow: Option<OAuthAuthorizeFlow>,
}
