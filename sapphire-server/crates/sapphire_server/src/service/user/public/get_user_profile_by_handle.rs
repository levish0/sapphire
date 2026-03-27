use crate::repository::user::get_by_handle::repository_get_user_by_handle;
use crate::repository::user::user_roles::repository_find_user_roles;
use crate::utils::r2_url::build_r2_public_url;
use sapphire_dto::user::PublicUserProfile;
use sapphire_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;

///
///
/// - `repository_get_user_by_handle`
/// - `repository_find_user_roles`
///
/// # Errors
pub async fn service_get_user_profile_by_handle(
    db: &DatabaseConnection,
    handle: &str,
) -> ServiceResult<PublicUserProfile> {
    let user = repository_get_user_by_handle(db, handle.to_string()).await?;
    let roles = repository_find_user_roles(db, user.id).await?;

    let profile = PublicUserProfile {
        id: user.id,
        handle: user.handle,
        display_name: user.display_name,
        bio: user.bio,
        profile_image: user.profile_image.as_deref().map(build_r2_public_url),
        banner_image: user.banner_image.as_deref().map(build_r2_public_url),
        roles,
        created_at: user.created_at,
    };

    Ok(profile)
}
