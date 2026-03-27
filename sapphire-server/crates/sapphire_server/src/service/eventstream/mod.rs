//! EventStream service layer.
//!
//! Exposes SSE stream constructors for realtime action-log delivery.

mod stream_actions;

pub use stream_actions::*;
