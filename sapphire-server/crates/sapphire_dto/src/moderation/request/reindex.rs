use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
/// Request enum for reindex entity type.
pub enum ReindexEntityType {
    Documents,
    Users,
    Discussions,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
/// Request payload for start reindex request.
pub struct StartReindexRequest {
    pub entity_type: ReindexEntityType,
    #[validate(range(
        min = 100,
        max = 50000,
        message = "Batch size must be between 100 and 50,000"
    ))]
    pub batch_size: Option<u32>,
}
