use crate::extractors::RequiredSession;
use crate::service::auth::totp::service_regenerate_backup_codes;
use crate::state::AppState;
use axum::extract::State;
use sapphire_dto::auth::request::TotpRegenerateBackupCodesRequest;
use sapphire_dto::auth::response::TotpBackupCodesResponse;
use sapphire_dto::validator::json_validator::ValidatedJson;
use sapphire_errors::errors::{ErrorResponse, Errors};

#[utoipa::path(
    post,
    path = "/v0/auth/totp/backup-codes/regenerate",
    request_body = TotpRegenerateBackupCodesRequest,
    responses(
        (status = 200, description = "Backup codes regenerated", body = TotpBackupCodesResponse),
        (status = 400, description = "Invalid TOTP code or TOTP not enabled", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth - TOTP"
)]
pub async fn totp_regenerate_backup_codes(
    State(state): State<AppState>,
    RequiredSession(session): RequiredSession,
    ValidatedJson(payload): ValidatedJson<TotpRegenerateBackupCodesRequest>,
) -> Result<TotpBackupCodesResponse, Errors> {
    service_regenerate_backup_codes(&state.db, session.user_id, &payload.code).await
}
