use crate::service::search::search_users::service_search_users;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::search::{SearchUsersRequest, SearchUsersResponse};
use sapphire_dto::validator::query_validator::ValidatedQuery;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    get,
    path = "/v0/search/users",
    params(SearchUsersRequest),
    responses(
        (status = 200, description = "User search results", body = SearchUsersResponse),
        (status = 400, description = "Bad request - Invalid query parameters or validation error", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    tag = "Search"
)]
pub async fn search_users(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<SearchUsersRequest>,
) -> Result<SearchUsersResponse, Errors> {
    let response = service_search_users(&state.meilisearch_client, &payload).await?;
    Ok(response)
}
