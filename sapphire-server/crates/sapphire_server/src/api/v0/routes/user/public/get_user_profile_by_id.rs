use crate::service::user::public::get_user_profile_by_id::service_get_user_profile_by_id;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::user::{GetUserProfileByIdRequest, PublicUserProfile};
use sapphire_dto::validator::query_validator::ValidatedQuery;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    get,
    path = "/v0/users/profile/id",
    params(GetUserProfileByIdRequest),
    responses(
        (status = 200, description = "User profile retrieved successfully", body = PublicUserProfile),
        (status = 400, description = "Bad request - Invalid query parameters", body = ErrorResponse),
        (status = 404, description = "Not Found - User not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error - Database error", body = ErrorResponse)
    ),
    tag = "User"
)]
pub async fn get_user_profile_by_id(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<GetUserProfileByIdRequest>,
) -> Result<PublicUserProfile, Errors> {
    let profile = service_get_user_profile_by_id(&state.db, payload.user_id).await?;
    Ok(profile)
}
