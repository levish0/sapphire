use crate::state::AppState;
use axum::{Router, routing::get};

use super::actions::stream_actions;

pub fn eventstream_routes() -> Router<AppState> {
    Router::new().route("/eventstream/actions", get(stream_actions))
}
