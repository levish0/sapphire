use chrono::{DateTime, Utc};
use validator::ValidationError;

pub fn validate_future_datetime(dt: &DateTime<Utc>) -> Result<(), ValidationError> {
    if *dt <= Utc::now() {
        return Err(ValidationError::new("expires_at_must_be_in_future"));
    }
    Ok(())
}
