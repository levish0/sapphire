//! Cursor-neighbor existence checks for moderation logs.
//!
//! Used by cursor pagination to calculate `has_newer` and `has_older`.

mod newer;
mod older;

pub use newer::repository_exists_newer_moderation_log;
pub use older::repository_exists_older_moderation_log;
