use sapphire_constants::ActionLogAction;
use sapphire_entity::common::ActionResourceType;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams, Validate)]
#[into_params(parameter_in = Query)]
/// Request payload for stream actions query.
pub struct StreamActionsQuery {
    /// Filter by user ID (for contributions)
    pub user_id: Option<Uuid>,
    /// Filter by resource type
    pub resource_type: Option<ActionResourceType>,
    /// Filter by resource ID
    pub resource_id: Option<Uuid>,
    /// Filter by actions
    pub actions: Option<Vec<ActionLogAction>>,
}
