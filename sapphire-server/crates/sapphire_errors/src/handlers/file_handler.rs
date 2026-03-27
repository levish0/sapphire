use crate::errors::Errors;
use crate::protocol::file::*;
use axum::http::StatusCode;
use tracing::warn;

pub fn log_error(error: &Errors) {
    match error {
        Errors::FileUploadError(_) | Errors::FileNotFound | Errors::FileReadError(_) => {
            warn!(error = ?error, "File/processing error");
        }

        _ => {}
    }
}

/// Returns: (StatusCode, error_code, details)
pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::FileUploadError(msg) => Some((
            StatusCode::BAD_REQUEST,
            FILE_UPLOAD_ERROR,
            Some(msg.clone()),
        )),
        Errors::FileNotFound => Some((StatusCode::NOT_FOUND, FILE_NOT_FOUND, None)),
        Errors::FileReadError(msg) => Some((
            StatusCode::INTERNAL_SERVER_ERROR,
            FILE_READ_ERROR,
            Some(msg.clone()),
        )),

        _ => None,
    }
}
