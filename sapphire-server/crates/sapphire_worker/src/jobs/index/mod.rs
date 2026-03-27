pub mod post;
pub mod user;

pub use post::{
    IndexPostJob, POSTS_INDEX, PostIndexAction, build_post_search_json,
    ensure_index_settings as ensure_post_index_settings,
};
pub use user::{
    IndexUserJob, USERS_INDEX, UserIndexAction, build_user_search_json,
    ensure_index_settings as ensure_user_index_settings,
};

/// Initialize all MeiliSearch indexes on worker startup.
pub async fn initialize_all_indexes(
    client: &meilisearch_sdk::client::Client,
) -> Result<(), anyhow::Error> {
    tracing::info!("Initializing MeiliSearch indexes");

    ensure_user_index_settings(client).await?;
    tracing::info!(index = USERS_INDEX, "Search index ready");

    ensure_post_index_settings(client).await?;
    tracing::info!(index = POSTS_INDEX, "Search index ready");

    tracing::info!("All MeiliSearch indexes initialized");
    Ok(())
}
