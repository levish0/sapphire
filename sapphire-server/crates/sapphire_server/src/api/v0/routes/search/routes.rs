use crate::state::AppState;
use axum::{Router, routing::get};

use super::search_users::search_users;

pub fn search_routes() -> Router<AppState> {
    Router::new().route("/search/users", get(search_users))
}
