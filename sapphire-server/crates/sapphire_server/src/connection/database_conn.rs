use anyhow::Result;
use sapphire_config::ServerConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::{error, info};

/// Establishes a database connection via PgDog connection pooler.
pub async fn establish_connection() -> Result<DatabaseConnection> {
    let config = ServerConfig::get();

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    );

    // Log with masked password
    let masked_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user,
        "*".repeat(config.db_password.len()),
        config.db_host,
        config.db_port,
        config.db_name
    );
    info!("Attempting to connect to database: {}", masked_url);

    // Configure connection options
    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(config.db_max_connection)
        .min_connections(config.db_min_connection)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(false);

    Database::connect(options)
        .await
        .map(|connection| {
            info!("Successfully connected to the database.");
            connection
        })
        .map_err(|err| {
            error!("Failed to connect to the database: {}", err);
            anyhow::Error::new(err).context("Failed to connect to the database")
        })
}
