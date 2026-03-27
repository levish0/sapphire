use anyhow::Result;
use futures::FutureExt;
use sapphire_worker::clients;
use sapphire_worker::config::WorkerConfig;
use sapphire_worker::connection;
use sapphire_worker::jobs::{self, WorkerContext};
use sapphire_worker::nats::streams::initialize_all_streams;
use sapphire_worker::utils;
use sapphire_worker::{CacheClient, DbPool};
use std::any::Any;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::{error, info};

const CONSUMER_RESTART_DELAY: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, Debug)]
enum ConsumerKind {
    Email,
    IndexUser,
    IndexPost,
    ReindexUsers,
}

impl ConsumerKind {
    const ALL: [Self; 4] = [
        Self::Email,
        Self::IndexUser,
        Self::IndexPost,
        Self::ReindexUsers,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::IndexUser => "index_user",
            Self::IndexPost => "index_post",
            Self::ReindexUsers => "reindex_users",
        }
    }
}

enum ConsumerExitOutcome {
    Completed(Result<()>),
    Panicked(String),
}

struct ConsumerExit {
    kind: ConsumerKind,
    outcome: ConsumerExitOutcome,
}

fn panic_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "unknown panic payload".to_string()
    }
}

async fn run_consumer(kind: ConsumerKind, ctx: WorkerContext) -> Result<()> {
    match kind {
        ConsumerKind::Email => jobs::email::run_consumer(ctx).await,
        ConsumerKind::IndexUser => jobs::index::user::run_consumer(ctx).await,
        ConsumerKind::IndexPost => jobs::index::post::run_consumer(ctx).await,
        ConsumerKind::ReindexUsers => jobs::reindex::users::run_consumer(ctx).await,
    }
}

fn spawn_consumer(consumers: &mut JoinSet<ConsumerExit>, kind: ConsumerKind, ctx: WorkerContext) {
    consumers.spawn(async move {
        let result = AssertUnwindSafe(run_consumer(kind, ctx))
            .catch_unwind()
            .await;

        let outcome = match result {
            Ok(result) => ConsumerExitOutcome::Completed(result),
            Err(panic_payload) => ConsumerExitOutcome::Panicked(panic_message(panic_payload)),
        };

        ConsumerExit { kind, outcome }
    });

    info!(consumer = kind.name(), "Consumer task started");
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = WorkerConfig::get();
    utils::logger::init_tracing();

    info!("Starting sapphire-worker...");

    let mailer = clients::create_mailer(config)?;
    let meili_client = clients::create_meili_client(config);
    jobs::index::initialize_all_indexes(&meili_client).await?;

    info!("Connecting to database...");
    let db_conn = connection::establish_connection().await?;
    let db_pool: DbPool = Arc::new(db_conn);
    info!("Shared clients created");

    let redis_cache_url = config.redis_cache_url();
    info!(url = %redis_cache_url, "Connecting to Redis Cache");
    let redis_cache_client = redis::Client::open(redis_cache_url)?;
    let redis_cache_conn = redis::aio::ConnectionManager::new(redis_cache_client).await?;
    let cache_client: CacheClient = Arc::new(redis_cache_conn);

    info!("Connecting to R2 assets...");
    let r2_assets = connection::establish_r2_assets_connection(config).await?;

    info!(url = %config.nats_url, "Connecting to NATS");
    let nats_client = async_nats::connect(&config.nats_url).await?;
    let jetstream = async_nats::jetstream::new(nats_client);
    let jetstream = Arc::new(jetstream);

    info!("Initializing JetStream streams...");
    initialize_all_streams(&jetstream).await?;

    let ctx = WorkerContext {
        mailer,
        meili_client,
        db_pool,
        cache_client,
        r2_assets,
        jetstream,
        config,
    };

    info!("Starting job consumers...");
    let mut consumers = JoinSet::new();
    for kind in ConsumerKind::ALL {
        spawn_consumer(&mut consumers, kind, ctx.clone());
    }
    info!("Job consumers started");

    info!("Starting cron scheduler...");
    let _cron_scheduler =
        jobs::cron::start_scheduler(ctx.db_pool.clone(), ctx.cache_client.clone(), config).await?;

    info!("All workers running");

    loop {
        let exited_consumer = match consumers.join_next().await {
            Some(Ok(exit)) => exit,
            Some(Err(e)) => {
                error!(error = ?e, "Failed to join consumer task");
                continue;
            }
            None => break,
        };

        match exited_consumer.outcome {
            ConsumerExitOutcome::Completed(Ok(())) => {
                error!(
                    consumer = exited_consumer.kind.name(),
                    "Consumer exited unexpectedly"
                );
            }
            ConsumerExitOutcome::Completed(Err(e)) => {
                error!(
                    consumer = exited_consumer.kind.name(),
                    error = %e,
                    "Consumer stopped with error"
                );
            }
            ConsumerExitOutcome::Panicked(panic) => {
                error!(
                    consumer = exited_consumer.kind.name(),
                    panic = %panic,
                    "Consumer panicked"
                );
            }
        }

        tokio::time::sleep(CONSUMER_RESTART_DELAY).await;
        info!(
            consumer = exited_consumer.kind.name(),
            restart_delay_secs = CONSUMER_RESTART_DELAY.as_secs(),
            "Restarting consumer"
        );
        spawn_consumer(&mut consumers, exited_consumer.kind, ctx.clone());
    }

    error!("Worker terminated unexpectedly");
    Ok(())
}
