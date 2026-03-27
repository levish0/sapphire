use super::action_logs::openapi::ActionLogsOpenApi;
use super::auth::openapi::AuthApiDoc;
use super::eventstream::openapi::EventStreamOpenApi;
use super::moderation::openapi::ModerationApiDoc;
use super::search::openapi::SearchApiDoc;
use super::user::openapi::UserApiDoc;
use super::user_preferences::openapi::UserPreferencesApiDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi()]
pub struct V0ApiDoc;

impl V0ApiDoc {
    pub fn merged() -> utoipa::openapi::OpenApi {
        let mut openapi = Self::openapi();
        openapi.merge(AuthApiDoc::openapi());
        openapi.merge(UserApiDoc::openapi());
        openapi.merge(SearchApiDoc::openapi());
        openapi.merge(ModerationApiDoc::openapi());

        openapi.merge(ActionLogsOpenApi::openapi());
        openapi.merge(UserPreferencesApiDoc::openapi());
        openapi.merge(EventStreamOpenApi::openapi());
        openapi
    }
}
