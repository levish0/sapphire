use validator::ValidationError;

/// Validates that a string is not blank (not empty after trimming whitespace)
pub fn validate_not_blank(s: &str) -> Result<(), ValidationError> {
    if s.trim().is_empty() {
        return Err(ValidationError::new("blank_string"));
    }
    Ok(())
}
