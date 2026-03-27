use sapphire_dto::moderation::{
    ListModerationLogsRequest, ListModerationLogsResponse, ModerationLogListItem,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::list_moderation_logs::list_moderation_logs,
    ),
    components(
        schemas(
            ListModerationLogsRequest,
            ListModerationLogsResponse,
            ModerationLogListItem,
        )
    ),
    tags(
        (name = "Moderation", description = "Moderation endpoints (admin actions)")
    )
)]
pub struct ModerationApiDoc;
