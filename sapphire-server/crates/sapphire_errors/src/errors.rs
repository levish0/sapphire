use crate::handlers::{
    email_handler, eventstream_handler, file_handler, general_handler, meilisearch_handler,
    oauth_handler, password_handler, rate_limit_handler, report_handler, session_handler,
    system_handler, token_handler, totp_handler, turnstile_handler, user_handler, worker_handler,
};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sapphire_config::ServerConfig;
use sea_orm::{DbErr, TransactionError};
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

pub type ServiceResult<T> = Result<T, Errors>;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: u16,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl From<DbErr> for Errors {
    fn from(err: DbErr) -> Self {
        Errors::DatabaseError(err.to_string())
    }
}

impl From<TransactionError<DbErr>> for Errors {
    fn from(err: TransactionError<DbErr>) -> Self {
        Errors::TransactionError(err.to_string())
    }
}

#[derive(Debug)]
pub enum Errors {
    // Auth errors
    InvalidCredentials,

    // User errors
    UserInvalidPassword,
    UserPasswordNotSet,
    UserInvalidSession,
    UserNotVerified,
    UserNotFound,
    UserUnauthorized,
    UserBanned,
    UserPermissionInsufficient,
    UserHandleAlreadyExists,
    UserEmailAlreadyExists,
    UserNotBanned,
    UserAlreadyBanned,
    UserDoesNotHaveRole,
    UserAlreadyHasRole,
    CannotManageSelf,
    CannotManageHigherOrEqualRole,
    UserTokenExpired,
    UserNoRefreshToken,
    UserInvalidToken,

    // Session errors
    SessionInvalidUserId,
    SessionExpired,
    SessionNotFound,

    // Permission errors
    ForbiddenError(String),

    // Post
    PostNotFound,

    // Report
    ReportNotFound,
    ReportAlreadyProcessed,
    ReportAlreadyExists,

    // OAuth
    OauthInvalidAuthUrl,
    OauthInvalidTokenUrl,
    OauthInvalidRedirectUrl,
    OauthTokenExchangeFailed,
    OauthUserInfoFetchFailed,
    OauthUserInfoParseFailed(String),
    OauthAccountAlreadyLinked,
    OauthConnectionNotFound,
    OauthCannotUnlinkLastConnection,
    OauthInvalidImageUrl,
    OauthInvalidState,
    OauthStateExpired,
    OauthHandleRequired,
    OauthEmailAlreadyExists,
    OauthEmailNotVerified,

    // Password errors
    PasswordRequiredForUpdate,
    PasswordIncorrect,
    PasswordCannotUpdateOauthOnly,
    PasswordNewPasswordMissing,
    PasswordAlreadySet,

    // Token errors
    TokenInvalidVerification,
    TokenExpiredVerification,
    TokenEmailMismatch,
    TokenInvalidReset,
    TokenExpiredReset,
    TokenInvalidEmailChange,

    // Email errors
    EmailAlreadyVerified,

    // File errors
    FileUploadError(String),
    FileNotFound,
    FileReadError(String),

    // Worker Service errors
    WorkerServiceConnectionFailed,
    WorkerServiceResponseInvalid,
    VerificationEmailSendFailed,
    PasswordResetEmailSendFailed,

    // EventStream errors
    EventStreamPublishFailed,

    // Comment errors
    CommentNotFound,
    InvalidParentComment,
    CannotReplyToDeletedComment,

    // Pin errors
    MessageAlreadyPinned,

    // General errors
    BadRequestError(String),
    ValidationError(String),
    FileTooLargeError(String),
    InvalidIpAddress,

    // System errors
    SysInternalError(String),
    DatabaseError(String),
    TransactionError(String),
    NotFound(String),
    HashingError(String),
    TokenCreationError(String),

    // Rate Limiting
    RateLimitExceeded,

    // Turnstile
    TurnstileTokenMissing,
    TurnstileVerificationFailed,
    TurnstileServiceError,

    // MeiliSearch
    MeiliSearchQueryFailed,

    // TOTP 2FA
    TotpAlreadyEnabled,
    TotpNotEnabled,
    TotpInvalidCode,
    TotpTempTokenInvalid,
    TotpTempTokenExpired,
    TotpBackupCodeExhausted,
    TotpSecretGenerationFailed,
    TotpQrGenerationFailed,
}

impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        // Central logging via domain handlers
        user_handler::log_error(&self);
        oauth_handler::log_error(&self);
        session_handler::log_error(&self);
        password_handler::log_error(&self);
        token_handler::log_error(&self);
        totp_handler::log_error(&self);
        email_handler::log_error(&self);
        file_handler::log_error(&self);
        worker_handler::log_error(&self);
        eventstream_handler::log_error(&self);
        report_handler::log_error(&self);
        rate_limit_handler::log_error(&self);
        turnstile_handler::log_error(&self);
        meilisearch_handler::log_error(&self);
        system_handler::log_error(&self);
        general_handler::log_error(&self);

        // HTTP response mapping via domain handlers
        let (status, code, details) = user_handler::map_response(&self)
            .or_else(|| oauth_handler::map_response(&self))
            .or_else(|| session_handler::map_response(&self))
            .or_else(|| password_handler::map_response(&self))
            .or_else(|| token_handler::map_response(&self))
            .or_else(|| totp_handler::map_response(&self))
            .or_else(|| email_handler::map_response(&self))
            .or_else(|| file_handler::map_response(&self))
            .or_else(|| worker_handler::map_response(&self))
            .or_else(|| eventstream_handler::map_response(&self))
            .or_else(|| report_handler::map_response(&self))
            .or_else(|| rate_limit_handler::map_response(&self))
            .or_else(|| turnstile_handler::map_response(&self))
            .or_else(|| meilisearch_handler::map_response(&self))
            .or_else(|| system_handler::map_response(&self))
            .or_else(|| general_handler::map_response(&self))
            .unwrap_or_else(|| {
                error!(error = ?self, "Unhandled error");
                (StatusCode::INTERNAL_SERVER_ERROR, "UNKNOWN_ERROR", None)
            });

        let details = if status.is_server_error() && !ServerConfig::get().is_dev {
            None
        } else {
            details
        };

        let body = ErrorResponse {
            status: status.as_u16(),
            code: code.to_string(),
            details,
        };

        (status, Json(body)).into_response()
    }
}

pub async fn handler_404<B>(req: axum::extract::Request<B>) -> impl IntoResponse {
    let path = req.uri().path();
    let method = req.method().to_string();

    Errors::NotFound(format!("Path {} with method {} not found", path, method))
}
