//! User preference keys enum for type-safe preference management

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;

/// User preference keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub enum UserPreferenceKey {
    /// Theme: "light" | "dark" | "system"
    #[serde(rename = "theme")]
    Theme,
}

impl UserPreferenceKey {
    /// Convert to database string value
    pub fn as_str(&self) -> &'static str {
        match self {
            UserPreferenceKey::Theme => "theme",
        }
    }

    /// Get all preference keys
    pub fn all() -> &'static [UserPreferenceKey] {
        &[UserPreferenceKey::Theme]
    }
}

impl fmt::Display for UserPreferenceKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for UserPreferenceKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "theme" => Ok(UserPreferenceKey::Theme),
            _ => Err(format!("Unknown user preference key: {}", s)),
        }
    }
}

/// Convert UserPreferenceKey to String for DB storage
pub fn user_preference_key_to_string(key: UserPreferenceKey) -> String {
    key.as_str().to_string()
}

/// Convert String from DB to UserPreferenceKey
pub fn string_to_user_preference_key(s: &str) -> Option<UserPreferenceKey> {
    s.parse().ok()
}
