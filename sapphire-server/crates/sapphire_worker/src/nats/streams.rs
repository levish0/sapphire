use async_nats::jetstream::{
    Context as JetStream,
    stream::{Config as StreamConfig, RetentionPolicy},
};

// Stream names
/// JetStream stream name for email jobs.
pub const EMAIL_STREAM: &str = "sapphire_jobs_email";
/// JetStream stream name for index user jobs.
pub const INDEX_USER_STREAM: &str = "sapphire_jobs_index_user";
/// JetStream stream name for index post jobs.
pub const INDEX_POST_STREAM: &str = "sapphire_jobs_index_post";
/// JetStream stream name for reindex users jobs.
pub const REINDEX_USERS_STREAM: &str = "sapphire_jobs_reindex_users";

// Subjects (for publishing)
/// NATS subject used to publish email jobs.
pub const EMAIL_SUBJECT: &str = "sapphire.jobs.email";
/// NATS subject used to publish index user jobs.
pub const INDEX_USER_SUBJECT: &str = "sapphire.jobs.index.user";
/// NATS subject used to publish index post jobs.
pub const INDEX_POST_SUBJECT: &str = "sapphire.jobs.index.post";
/// NATS subject used to publish reindex users jobs.
pub const REINDEX_USERS_SUBJECT: &str = "sapphire.jobs.reindex.users";

// Consumer names
/// Durable consumer name for email jobs.
pub const EMAIL_CONSUMER: &str = "email-consumer";
/// Durable consumer name for index user jobs.
pub const INDEX_USER_CONSUMER: &str = "user-index-consumer";
/// Durable consumer name for index post jobs.
pub const INDEX_POST_CONSUMER: &str = "post-index-consumer";
/// Durable consumer name for reindex users jobs.
pub const REINDEX_USERS_CONSUMER: &str = "reindex-users-consumer";

/// Stream and subject pairs for initialization
const STREAMS: &[(&str, &str)] = &[
    (EMAIL_STREAM, EMAIL_SUBJECT),
    (INDEX_USER_STREAM, INDEX_USER_SUBJECT),
    (INDEX_POST_STREAM, INDEX_POST_SUBJECT),
    (REINDEX_USERS_STREAM, REINDEX_USERS_SUBJECT),
];

/// Initialize all job streams with WorkQueue retention policy
pub async fn initialize_all_streams(jetstream: &JetStream) -> anyhow::Result<()> {
    for (stream_name, subject) in STREAMS {
        jetstream
            .get_or_create_stream(StreamConfig {
                name: stream_name.to_string(),
                subjects: vec![subject.to_string()],
                retention: RetentionPolicy::WorkQueue,
                ..Default::default()
            })
            .await?;
        tracing::info!("Stream {} ready (subject: {})", stream_name, subject);
    }
    Ok(())
}
