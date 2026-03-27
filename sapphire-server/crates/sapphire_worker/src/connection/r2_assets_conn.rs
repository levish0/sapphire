use crate::config::WorkerConfig;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::{Client, Error as S3Error};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::info;

/// Object metadata from storage listing
#[derive(Debug, Clone)]
pub struct StorageObjectInfo {
    pub key: String,
    pub last_modified: Option<DateTime<Utc>>,
}

// Cloudflare R2
#[derive(Clone)]
/// Connection client type for r2 assets client.
pub struct R2AssetsClient {
    client: Arc<Client>,
    bucket: String,
    public_domain: String,
}

impl R2AssetsClient {
    /// Helper function for new.
    pub fn new(client: Client, bucket: String, public_domain: String) -> Self {
        Self {
            client: Arc::new(client),
            bucket,
            public_domain,
        }
    }

    /// Helper function for upload.
    pub async fn upload(&self, key: &str, body: Vec<u8>) -> Result<(), S3Error> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body.into())
            .send()
            .await?;
        Ok(())
    }

    /// Helper function for upload with content type.
    pub async fn upload_with_content_type(
        &self,
        key: &str,
        body: Vec<u8>,
        content_type: &str,
    ) -> Result<(), aws_sdk_s3::Error> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body.into())
            .content_type(content_type)
            .send()
            .await?;
        Ok(())
    }

    /// Helper function for download.
    pub async fn download(&self, key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let data = resp.body.collect().await?;
        Ok(data.into_bytes().to_vec())
    }

    /// Helper function for delete.
    pub async fn delete(&self, key: &str) -> Result<(), S3Error> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        Ok(())
    }

    /// Helper function for exists.
    pub async fn exists(&self, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => match &err {
                SdkError::ServiceError(service_err) => {
                    if service_err.err().is_not_found() {
                        Ok(false)
                    } else {
                        Err(Box::new(err))
                    }
                }
                _ => Err(Box::new(err)),
            },
        }
    }

    /// Helper function for upload file.
    pub async fn upload_file(
        &self,
        key: &str,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_content = tokio::fs::read(file_path).await?;
        self.upload(key, file_content).await?;
        Ok(())
    }

    /// Returns public url.
    pub fn get_public_url(&self, key: &str) -> String {
        format!("{}/{}", self.public_domain, key)
    }

    /// List objects with a given prefix (keys only)
    pub async fn list_objects_by_prefix(&self, prefix: &str) -> Result<Vec<String>, S3Error> {
        let mut keys = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let mut request = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix);

            if let Some(token) = continuation_token.as_deref() {
                request = request.continuation_token(token);
            }

            let resp = request.send().await?;

            keys.extend(
                resp.contents()
                    .iter()
                    .filter_map(|obj| obj.key().map(|k| k.to_string())),
            );

            continuation_token = resp.next_continuation_token().map(|s| s.to_string());
            if continuation_token.is_none() {
                break;
            }
        }

        Ok(keys)
    }

    /// List objects with pagination and last_modified info
    pub async fn list_objects(
        &self,
        continuation_token: Option<&str>,
        max_keys: i32,
    ) -> Result<(Vec<StorageObjectInfo>, Option<String>), Box<dyn std::error::Error + Send + Sync>>
    {
        let mut request = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .max_keys(max_keys);

        if let Some(token) = continuation_token {
            request = request.continuation_token(token);
        }

        let resp = request.send().await?;

        let objects: Vec<StorageObjectInfo> = resp
            .contents()
            .iter()
            .filter_map(|obj| {
                let key = obj.key()?.to_string();
                let last_modified = obj.last_modified().map(|t| {
                    DateTime::from_timestamp(t.secs(), t.subsec_nanos()).unwrap_or_default()
                });
                Some(StorageObjectInfo { key, last_modified })
            })
            .collect();

        let next_token = resp.next_continuation_token().map(|s| s.to_string());

        Ok((objects, next_token))
    }

    /// List objects under a specific prefix with pagination and last_modified info.
    pub async fn list_objects_with_prefix(
        &self,
        prefix: &str,
        continuation_token: Option<&str>,
        max_keys: i32,
    ) -> Result<(Vec<StorageObjectInfo>, Option<String>), Box<dyn std::error::Error + Send + Sync>>
    {
        let mut request = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(prefix)
            .max_keys(max_keys);

        if let Some(token) = continuation_token {
            request = request.continuation_token(token);
        }

        let resp = request.send().await?;

        let objects: Vec<StorageObjectInfo> = resp
            .contents()
            .iter()
            .filter_map(|obj| {
                let key = obj.key()?.to_string();
                let last_modified = obj.last_modified().map(|t| {
                    DateTime::from_timestamp(t.secs(), t.subsec_nanos()).unwrap_or_default()
                });
                Some(StorageObjectInfo { key, last_modified })
            })
            .collect();

        let next_token = resp.next_continuation_token().map(|s| s.to_string());

        Ok((objects, next_token))
    }
}

/// Helper function for establish r2 assets connection.
pub async fn establish_r2_assets_connection(
    config: &WorkerConfig,
) -> anyhow::Result<R2AssetsClient> {
    info!(
        "Connecting to R2 assets at: {} (region: {})",
        config.r2_endpoint, config.r2_region
    );

    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(config.r2_region.clone()))
        .endpoint_url(&config.r2_endpoint)
        .credentials_provider(aws_sdk_s3::config::Credentials::new(
            &config.r2_access_key_id,
            &config.r2_secret_access_key,
            None,
            None,
            "r2-credentials",
        ))
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&aws_config)
        .force_path_style(true)
        .build();

    let client = Client::from_conf(s3_config);
    let r2_client = R2AssetsClient::new(
        client,
        config.r2_assets_bucket_name.clone(),
        config.r2_assets_public_domain.clone(),
    );

    info!("Successfully connected to R2 assets");
    Ok(r2_client)
}
