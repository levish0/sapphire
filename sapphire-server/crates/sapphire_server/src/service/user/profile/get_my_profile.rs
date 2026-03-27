use crate::repository::user::repository_get_user_by_id;
use crate::repository::user::user_roles::repository_find_user_roles;
use crate::service::auth::session_types::SessionContext;
use crate::utils::r2_url::build_r2_public_url;
use sapphire_dto::user::UserResponse;
use sapphire_errors::errors::Errors;
use sea_orm::DatabaseConnection;

///
///
/// - `repository_get_user_by_id`
/// - `repository_find_user_roles`
///
/// # Errors
pub async fn service_get_my_profile(
    db: &DatabaseConnection,
    session: &SessionContext,
) -> Result<UserResponse, Errors> {
    let user = repository_get_user_by_id(db, session.user_id).await?;
    let roles = repository_find_user_roles(db, session.user_id).await?;

    let response = UserResponse {
        id: session.user_id.to_string(),
        email: user.email,
        handle: user.handle,
        display_name: user.display_name,
        bio: user.bio,
        profile_image: user.profile_image.as_deref().map(build_r2_public_url),
        banner_image: user.banner_image.as_deref().map(build_r2_public_url),
        roles,
        created_at: user.created_at,
    };

    Ok(response)
}
