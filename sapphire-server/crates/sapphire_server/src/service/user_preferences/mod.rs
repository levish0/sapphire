//! User preference service layer.
//!
//! This module orchestrates user-specific preference CRUD and bulk updates
//! for authenticated users.

mod delete_preference;
mod get_preference;
mod get_preferences;
mod set_preference;
mod set_preferences_bulk;

pub use delete_preference::*;
pub use get_preference::*;
pub use get_preferences::*;
pub use set_preference::*;
pub use set_preferences_bulk::*;
