use crate::repository::user::find_by_handle::repository_find_user_by_handle;
use sapphire_dto::user::CheckHandleAvailableResponse;
use sapphire_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;

///
///
/// - `repository_find_user_by_handle`
///
/// # Errors
pub async fn service_check_handle_available(
    db: &DatabaseConnection,
    handle: &str,
) -> ServiceResult<CheckHandleAvailableResponse> {
    let user = repository_find_user_by_handle(db, handle.to_string()).await?;

    Ok(CheckHandleAvailableResponse {
        available: user.is_none(),
    })
}
