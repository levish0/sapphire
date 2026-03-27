use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, get, put},
};

use super::delete_preference::delete_user_preference;
use super::get_preference::get_user_preference;
use super::get_preferences::get_user_preferences;
use super::set_preference::set_user_preference;
use super::set_preferences_bulk::set_user_preferences_bulk;

pub fn user_preferences_routes() -> Router<AppState> {
    Router::new()
        .route("/preferences", get(get_user_preferences))
        .route("/preferences", put(set_user_preferences_bulk))
        .route("/preferences/{key}", get(get_user_preference))
        .route("/preferences/{key}", put(set_user_preference))
        .route("/preferences/{key}", delete(delete_user_preference))
}
