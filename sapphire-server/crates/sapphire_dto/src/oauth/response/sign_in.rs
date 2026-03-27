use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::auth::response::create_login_response;
use crate::oauth::internal::SignInResult;
use sapphire_errors::errors::Errors;

#[derive(Debug, Serialize, ToSchema)]
pub struct OAuthPendingSignupResponse {
    pub pending_token: String,
    pub email: String,
    pub display_name: String,
}

impl IntoResponse for OAuthPendingSignupResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub enum OAuthSignInResponse {
    Success {
        session_id: String,
    },
    /// Pending signup - 200 OK + JSON body
    PendingSignup(OAuthPendingSignupResponse),
}

impl OAuthSignInResponse {
    pub fn from_result(result: SignInResult) -> Self {
        match result {
            SignInResult::Success(session_id) => OAuthSignInResponse::Success { session_id },
            SignInResult::PendingSignup {
                pending_token,
                email,
                display_name,
            } => OAuthSignInResponse::PendingSignup(OAuthPendingSignupResponse {
                pending_token,
                email,
                display_name,
            }),
        }
    }

    pub fn into_response_result(self) -> Result<axum::response::Response, Errors> {
        match self {
            OAuthSignInResponse::Success { session_id } => create_login_response(session_id, true),
            OAuthSignInResponse::PendingSignup(response) => Ok(response.into_response()),
        }
    }
}
