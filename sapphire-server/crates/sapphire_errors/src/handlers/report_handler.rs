use crate::errors::Errors;
use crate::protocol::report::*;
use axum::http::StatusCode;
use tracing::debug;

pub fn log_error(error: &Errors) {
    match error {
        Errors::ReportNotFound | Errors::ReportAlreadyExists | Errors::ReportAlreadyProcessed => {
            debug!(error = ?error, "Report error");
        }

        _ => {}
    }
}

/// Returns: (StatusCode, error_code, details)
pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::ReportNotFound => Some((StatusCode::NOT_FOUND, REPORT_NOT_FOUND, None)),
        Errors::ReportAlreadyProcessed => {
            Some((StatusCode::CONFLICT, REPORT_ALREADY_PROCESSED, None))
        }
        Errors::ReportAlreadyExists => Some((StatusCode::CONFLICT, REPORT_ALREADY_EXISTS, None)),

        _ => None,
    }
}
