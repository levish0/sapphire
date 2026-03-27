use super::publish_job;
use crate::state::WorkerClient;
use sapphire_errors::errors::Errors;
use sapphire_worker::jobs::email::{EmailTemplate, SendEmailJob};
use sapphire_worker::nats::streams::EMAIL_SUBJECT;
use tracing::info;

/// Push a verification email job to the worker queue
pub async fn send_verification_email(
    worker: &WorkerClient,
    email_to: &str,
    username: &str,
    verification_token: &str,
    valid_minutes: u64,
) -> Result<(), Errors> {
    let job = SendEmailJob {
        to: email_to.to_string(),
        subject: "Verify your email".to_string(),
        template: EmailTemplate::Verification {
            username: username.to_string(),
            token: verification_token.to_string(),
            valid_minutes,
        },
    };

    publish_job(worker, EMAIL_SUBJECT, &job).await?;

    info!(template = "verification", "Verification email job queued");
    Ok(())
}

/// Push a password reset email job to the worker queue
pub async fn send_password_reset_email(
    worker: &WorkerClient,
    email_to: &str,
    handle: &str,
    reset_token: &str,
    valid_minutes: u64,
) -> Result<(), Errors> {
    let job = SendEmailJob {
        to: email_to.to_string(),
        subject: "Reset your password".to_string(),
        template: EmailTemplate::PasswordReset {
            handle: handle.to_string(),
            token: reset_token.to_string(),
            valid_minutes,
        },
    };

    publish_job(worker, EMAIL_SUBJECT, &job).await?;

    info!(
        template = "password_reset",
        "Password reset email job queued"
    );
    Ok(())
}

/// Push an email change verification job to the worker queue
pub async fn send_email_change_verification(
    worker: &WorkerClient,
    new_email: &str,
    username: &str,
    token: &str,
    valid_minutes: u64,
) -> Result<(), Errors> {
    let job = SendEmailJob {
        to: new_email.to_string(),
        subject: "Confirm your email change".to_string(),
        template: EmailTemplate::EmailChange {
            username: username.to_string(),
            token: token.to_string(),
            valid_minutes,
        },
    };

    publish_job(worker, EMAIL_SUBJECT, &job).await?;

    info!(
        template = "email_change",
        "Email change verification job queued"
    );
    Ok(())
}
