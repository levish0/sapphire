//! Moderation service layer.
//!
//! Provides moderation log listing for the current sapphire moderation scope.

pub mod list_moderation_logs;

pub use list_moderation_logs::service_list_moderation_logs;
