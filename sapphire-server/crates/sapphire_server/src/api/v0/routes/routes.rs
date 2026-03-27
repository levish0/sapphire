use super::action_logs::routes::action_logs_routes as ActionLogsRoutes;
use super::auth::routes::auth_routes as AuthRoutes;
use super::eventstream::routes::eventstream_routes as EventStreamRoutes;
use super::moderation::routes::moderation_routes as ModerationRoutes;
use super::search::routes::search_routes as SearchRoutes;
use super::user::routes::user_routes as UserRoutes;
use super::user_preferences::routes::user_preferences_routes as UserPreferencesRoutes;
use crate::state::AppState;
use axum::Router;

pub fn v0_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(UserRoutes())
        .merge(AuthRoutes(state))
        .merge(SearchRoutes())
        .merge(ModerationRoutes())
        .merge(ActionLogsRoutes())
        .merge(UserPreferencesRoutes())
        .merge(EventStreamRoutes())
}
