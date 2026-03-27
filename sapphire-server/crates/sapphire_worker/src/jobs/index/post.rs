use crate::DbPool;
use crate::jobs::WorkerContext;
use crate::nats::consumer::NatsConsumer;
use crate::nats::streams::{INDEX_POST_CONSUMER, INDEX_POST_STREAM};
use sapphire_entity::posts;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexPostJob {
    pub post_id: Uuid,
    pub action: PostIndexAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostIndexAction {
    Index,
    Delete,
}

pub const POSTS_INDEX: &str = "posts";

/// Build a search document JSON from post model
pub fn build_post_search_json(post: &posts::Model) -> JsonValue {
    json!({
        "id": post.id.to_string(),
        "user_id": post.user_id.to_string(),
        "content": post.content,
        "created_at": post.created_at.timestamp(),
    })
}

fn post_index_settings() -> meilisearch_sdk::settings::Settings {
    meilisearch_sdk::settings::Settings::new()
        .with_searchable_attributes(["content"])
        .with_displayed_attributes(["id", "user_id", "content", "created_at"])
        .with_sortable_attributes(["created_at"])
        .with_ranking_rules([
            "words",
            "typo",
            "proximity",
            "attribute",
            "sort",
            "exactness",
        ])
}

async fn handle_index_post(
    job: IndexPostJob,
    client: &crate::SearchClient,
    db: &DbPool,
) -> Result<(), anyhow::Error> {
    tracing::info!(post_id = %job.post_id, action = ?job.action, "Processing post index job");

    let index = client.index(POSTS_INDEX);

    ensure_index_settings(client).await?;

    match job.action {
        PostIndexAction::Index => {
            let post = match posts::Entity::find_by_id(job.post_id)
                .one(db.as_ref())
                .await?
            {
                Some(post) => post,
                None => {
                    tracing::warn!(post_id = %job.post_id, "Post not found for indexing");
                    return Ok(());
                }
            };

            // Only index posts with content (skip pure reposts)
            if post.content.is_none() {
                tracing::debug!(post_id = %job.post_id, "Skipping pure repost (no content)");
                return Ok(());
            }

            let search_doc = build_post_search_json(&post);
            index.add_documents(&[search_doc], Some("id")).await?;
            tracing::info!(post_id = %job.post_id, "Post indexed successfully");
        }
        PostIndexAction::Delete => {
            index.delete_document(&job.post_id.to_string()).await?;
            tracing::info!(post_id = %job.post_id, "Post deleted from index");
        }
    }

    Ok(())
}

/// Ensure index exists with proper settings
pub async fn ensure_index_settings(
    client: &meilisearch_sdk::client::Client,
) -> Result<(), anyhow::Error> {
    let index = client.index(POSTS_INDEX);

    match index.get_stats().await {
        Ok(_) => Ok(()),
        Err(meilisearch_sdk::errors::Error::Meilisearch(ref e))
            if e.error_code == meilisearch_sdk::errors::ErrorCode::IndexNotFound =>
        {
            tracing::info!(index = POSTS_INDEX, "Creating search index");
            let task = client.create_index(POSTS_INDEX, Some("id")).await?;
            task.wait_for_completion(client, None, None).await?;

            tracing::info!(index = POSTS_INDEX, "Applying search index settings");
            let index = client.index(POSTS_INDEX);
            let task = index.set_settings(&post_index_settings()).await?;
            task.wait_for_completion(client, None, None).await?;

            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

/// Run the post index consumer
pub async fn run_consumer(ctx: WorkerContext) -> anyhow::Result<()> {
    let meili_client = ctx.meili_client.clone();
    let db_pool = ctx.db_pool.clone();

    let consumer = NatsConsumer::new(
        ctx.jetstream.clone(),
        INDEX_POST_STREAM,
        INDEX_POST_CONSUMER,
        2,
    );

    consumer
        .run::<IndexPostJob, _, _>(move |job| {
            let client = meili_client.clone();
            let db = db_pool.clone();
            async move { handle_index_post(job, &client, &db).await }
        })
        .await
}
