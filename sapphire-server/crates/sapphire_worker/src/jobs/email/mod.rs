use crate::Mailer;
use crate::config::WorkerConfig;
use crate::jobs::WorkerContext;
use crate::nats::consumer::NatsConsumer;
use crate::nats::streams::{EMAIL_CONSUMER, EMAIL_STREAM};
use lettre::message::{Mailbox, header::ContentType};
use lettre::{AsyncTransport, Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Worker job payload for send email job.
pub struct SendEmailJob {
    pub to: String,
    pub subject: String,
    pub template: EmailTemplate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enum describing email template.
pub enum EmailTemplate {
    Verification {
        username: String,
        token: String,
        valid_minutes: u64,
    },
    PasswordReset {
        handle: String,
        token: String,
        valid_minutes: u64,
    },
    EmailChange {
        username: String,
        token: String,
        valid_minutes: u64,
    },
    Custom {
        html_content: String,
    },
}

async fn handle_send_email(job: SendEmailJob, mailer: &Mailer) -> Result<(), anyhow::Error> {
    tracing::info!(subject = %job.subject, "Processing email job");

    let config = WorkerConfig::get();

    // Render HTML content based on template
    let html_content = render_template(&job.template)?;

    let from_mailbox: Mailbox =
        format!("{} <{}>", config.emails_from_name, config.emails_from_email).parse()?;
    let to_mailbox: Mailbox = job.to.parse()?;

    let message = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject(&job.subject)
        .header(ContentType::TEXT_HTML)
        .body(html_content)?;

    mailer.send(message).await?;

    tracing::info!(subject = %job.subject, "Email sent successfully");
    Ok(())
}

fn render_template(template: &EmailTemplate) -> Result<String, anyhow::Error> {
    let config = WorkerConfig::get();

    let html = match template {
        EmailTemplate::Verification {
            username,
            token,
            valid_minutes,
        } => {
            let verification_link = format!(
                "{}{}?token={}",
                config.frontend_host,
                config.frontend_path_verify_email,
                urlencoding::encode(token)
            );
            crate::templates::render_email_verification(
                &config.project_name,
                username,
                &verification_link,
                *valid_minutes,
            )
            .map_err(|e| anyhow::anyhow!("Template error: {}", e))?
        }
        EmailTemplate::PasswordReset {
            handle,
            token,
            valid_minutes,
        } => {
            let reset_link = format!(
                "{}{}?token={}",
                config.frontend_host,
                config.frontend_path_reset_password,
                urlencoding::encode(token)
            );
            crate::templates::render_password_reset(
                &config.project_name,
                handle,
                &reset_link,
                *valid_minutes,
            )
            .map_err(|e| anyhow::anyhow!("Template error: {}", e))?
        }
        EmailTemplate::EmailChange {
            username,
            token,
            valid_minutes,
        } => {
            let confirmation_link = format!(
                "{}{}?token={}",
                config.frontend_host,
                config.frontend_path_confirm_email_change,
                urlencoding::encode(token)
            );
            crate::templates::render_email_change(
                &config.project_name,
                username,
                &confirmation_link,
                *valid_minutes,
            )
            .map_err(|e| anyhow::anyhow!("Template error: {}", e))?
        }
        EmailTemplate::Custom { html_content } => html_content.clone(),
    };

    Ok(html)
}

/// Run the email consumer
pub async fn run_consumer(ctx: WorkerContext) -> anyhow::Result<()> {
    let mailer = ctx.mailer.clone();

    let consumer = NatsConsumer::new(
        ctx.jetstream.clone(),
        EMAIL_STREAM,
        EMAIL_CONSUMER,
        2, // concurrency
    );

    consumer
        .run::<SendEmailJob, _, _>(move |job| {
            let mailer = mailer.clone();
            async move { handle_send_email(job, &mailer).await }
        })
        .await
}
