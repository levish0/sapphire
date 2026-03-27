use sapphire_dto::user_preferences::{
    PreferenceItem, SetUserPreferenceRequest, SetUserPreferencesBulkRequest,
    UserPreferenceResponse, UserPreferencesResponse,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::get_preferences::get_user_preferences,
        super::get_preference::get_user_preference,
        super::set_preference::set_user_preference,
        super::set_preferences_bulk::set_user_preferences_bulk,
        super::delete_preference::delete_user_preference,
    ),
    components(
        schemas(
            UserPreferenceResponse,
            UserPreferencesResponse,
            SetUserPreferenceRequest,
            SetUserPreferencesBulkRequest,
            PreferenceItem,
        )
    ),
    tags(
        (name = "User Preferences", description = "User preferences endpoints")
    )
)]
pub struct UserPreferencesApiDoc;
