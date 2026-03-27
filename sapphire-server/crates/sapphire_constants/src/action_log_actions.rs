use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;

/// Action log actions stored in `action_logs.action`.
/// Format: `{resource}:{operation}`.
///
/// Moderation actions track moderator or administrator operations, while
/// action log actions track regular user activity such as create or edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ActionLogAction {
    // ==================== Post Actions (3) ====================
    #[serde(rename = "post:create")]
    PostCreate,
    #[serde(rename = "post:edit")]
    PostEdit,
    #[serde(rename = "post:delete")]
    PostDelete,
    // ==================== Comment Actions (3) ====================
    #[serde(rename = "comment:create")]
    CommentCreate,
    #[serde(rename = "comment:edit")]
    CommentEdit,
    #[serde(rename = "comment:delete")]
    CommentDelete,
}

impl ActionLogAction {
    /// Convert to the database string value.
    pub fn as_str(&self) -> &'static str {
        match self {
            // Post
            ActionLogAction::PostCreate => "post:create",
            ActionLogAction::PostEdit => "post:edit",
            ActionLogAction::PostDelete => "post:delete",
            // Comment
            ActionLogAction::CommentCreate => "comment:create",
            ActionLogAction::CommentEdit => "comment:edit",
            ActionLogAction::CommentDelete => "comment:delete",
        }
    }
}

impl fmt::Display for ActionLogAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ActionLogAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Post
            "post:create" => Ok(ActionLogAction::PostCreate),
            "post:edit" => Ok(ActionLogAction::PostEdit),
            "post:delete" => Ok(ActionLogAction::PostDelete),
            // Comment
            "comment:create" => Ok(ActionLogAction::CommentCreate),
            "comment:edit" => Ok(ActionLogAction::CommentEdit),
            "comment:delete" => Ok(ActionLogAction::CommentDelete),
            _ => Err(format!("Unknown action log action: {}", s)),
        }
    }
}

/// Convert an action log action to its stored string form.
pub fn action_log_action_to_string(action: ActionLogAction) -> String {
    action.as_str().to_string()
}

/// Parse an action log action from its stored string form.
pub fn string_to_action_log_action(s: &str) -> Option<ActionLogAction> {
    s.parse().ok()
}
