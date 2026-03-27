use sapphire_dto::search::{SearchUsersRequest, SearchUsersResponse, UserSearchHit};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::search_users::search_users,
    ),
    components(
        schemas(
            SearchUsersRequest,
            SearchUsersResponse,
            UserSearchHit,
        )
    ),
    tags(
        (name = "Search", description = "Search endpoints")
    )
)]
pub struct SearchApiDoc;
