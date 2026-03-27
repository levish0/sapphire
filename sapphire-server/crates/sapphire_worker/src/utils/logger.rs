use std::sync::LazyLock;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

static TRACING_GUARD: LazyLock<Option<tracing_appender::non_blocking::WorkerGuard>> =
    LazyLock::new(|| {
        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            #[cfg(debug_assertions)]
            let default = "debug";

            #[cfg(not(debug_assertions))]
            let default = "info";

            EnvFilter::new(default)
        });

        #[cfg(debug_assertions)]
        {
            tracing_subscriber::registry()
                .with(
                    fmt::layer()
                        .with_writer(std::io::stdout)
                        .with_filter(env_filter),
                )
                .init();

            info!("Tracing initialized (development mode: console only)");
            None
        }

        #[cfg(not(debug_assertions))]
        {
            tracing_subscriber::registry()
                .with(
                    fmt::layer()
                        .json()
                        .with_writer(std::io::stdout)
                        .with_filter(env_filter),
                )
                .init();

            info!("Tracing initialized (production mode: JSON stdout)");
            None
        }
    });

/// Initializes tracing.
pub fn init_tracing() {
    LazyLock::force(&TRACING_GUARD);
}
