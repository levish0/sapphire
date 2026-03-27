use crate::jobs::WorkerContext;
use crate::nats::consumer::NatsConsumer;
use crate::nats::streams::{INDEX_USER_CONSUMER, INDEX_USER_STREAM};
use crate::{DbPool, SearchClient};
use sapphire_entity::users;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Worker job payload for index user job.
pub struct IndexUserJob {
    pub user_id: Uuid,
    pub action: UserIndexAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enum describing user index action.
pub enum UserIndexAction {
    Index,
    Delete,
}

/// Constant value for users index.
pub const USERS_INDEX: &str = "users";

/// Build a search document JSON from user model
pub fn build_user_search_json(user: &users::Model) -> JsonValue {
    json!({
        "id": user.id.to_string(),
        "handle": user.handle,
        "display_name": user.display_name,
        "bio": user.bio,
        "profile_image": user.profile_image,
    })
}

/// MeiliSearch index settings for users
fn user_index_settings() -> meilisearch_sdk::settings::Settings {
    meilisearch_sdk::settings::Settings::new()
        .with_searchable_attributes(["handle", "display_name", "bio"])
        .with_displayed_attributes(["id", "handle", "display_name", "bio", "profile_image"])
        .with_ranking_rules(["words", "typo", "proximity", "attribute", "exactness"])
}

async fn handle_index_user(
    job: IndexUserJob,
    client: &SearchClient,
    db: &DbPool,
) -> Result<(), anyhow::Error> {
    tracing::info!(user_id = %job.user_id, action = ?job.action, "Processing user index job");

    let index = client.index(USERS_INDEX);

    // Ensure index exists and settings are applied
    ensure_index_settings(client).await?;

    match job.action {
        UserIndexAction::Index => {
            // Fetch user from DB
            let user = match users::Entity::find_by_id(job.user_id)
                .one(db.as_ref())
                .await?
            {
                Some(user) => user,
                None => {
                    tracing::warn!(user_id = %job.user_id, "User not found for indexing");
                    return Ok(());
                }
            };

            // Build search document
            let search_user = build_user_search_json(&user);

            // Add to index (upsert)
            index.add_documents(&[search_user], Some("id")).await?;
            tracing::info!(user_id = %job.user_id, "User indexed successfully");
        }
        UserIndexAction::Delete => {
            index.delete_document(&job.user_id.to_string()).await?;
            tracing::info!(user_id = %job.user_id, "User deleted from index");
        }
    }

    Ok(())
}

/// Ensure index exists with proper settings
pub async fn ensure_index_settings(
    client: &meilisearch_sdk::client::Client,
) -> Result<(), anyhow::Error> {
    let index = client.index(USERS_INDEX);

    // Check if index exists by trying to get stats
    match index.get_stats().await {
        Ok(_) => {
            // Index exists, settings should already be applied
            Ok(())
        }
        Err(meilisearch_sdk::errors::Error::Meilisearch(ref e))
            if e.error_code == meilisearch_sdk::errors::ErrorCode::IndexNotFound =>
        {
            // Index doesn't exist, create it
            tracing::info!(index = USERS_INDEX, "Creating search index");
            let task = client.create_index(USERS_INDEX, Some("id")).await?;
            task.wait_for_completion(client, None, None).await?;

            // Apply settings
            tracing::info!(index = USERS_INDEX, "Applying search index settings");
            let index = client.index(USERS_INDEX);
            let task = index.set_settings(&user_index_settings()).await?;
            task.wait_for_completion(client, None, None).await?;

            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

/// Run the user index consumer
pub async fn run_consumer(ctx: WorkerContext) -> anyhow::Result<()> {
    let meili_client = ctx.meili_client.clone();
    let db_pool = ctx.db_pool.clone();

    let consumer = NatsConsumer::new(
        ctx.jetstream.clone(),
        INDEX_USER_STREAM,
        INDEX_USER_CONSUMER,
        2, // concurrency
    );

    consumer
        .run::<IndexUserJob, _, _>(move |job| {
            let client = meili_client.clone();
            let db = db_pool.clone();
            async move { handle_index_user(job, &client, &db).await }
        })
        .await
}
