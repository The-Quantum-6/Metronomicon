//! Object storage backed by Garage (S3-compatible).
//!
//! `Storage` is a thin, cloneable wrapper around an S3 client plus the bucket
//! name. Build one with [`Storage::from_env`] at startup, keep it in the app
//! state, and handlers can pull it out with `State<Storage>`.

use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::primitives::ByteStream;

/// Errors from the storage layer. Kept small on purpose: the detailed message
/// is for logs, while `AppError` decides what the client actually sees.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("storage error: {0}")]
    Sdk(String),
}

/// A handle to the object store. Cloning is cheap (the inner S3 client is
/// reference-counted), so it is fine to keep one in the shared app state.
#[derive(Clone)]
pub struct Storage {
    client: Client,
    bucket: String,
}

impl Storage {
    /// Build a client from environment variables: `S3_ENDPOINT`, `S3_REGION`,
    /// `S3_BUCKET`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`.
    ///
    /// This only constructs the client; it makes no network calls, so it is
    /// safe to run at startup even when Garage is not up yet.
    pub async fn from_env() -> Self {
        let endpoint =
            std::env::var("S3_ENDPOINT").unwrap_or_else(|_| "http://localhost:3900".into());
        let region = std::env::var("S3_REGION").unwrap_or_else(|_| "garage".into());
        let bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "test-bucket".into());

        let base = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(region))
            .endpoint_url(endpoint)
            .load()
            .await;

        // Garage only supports path-style addressing (host/bucket), not
        // virtual-hosted-style (bucket.host), so force it here.
        let config = aws_sdk_s3::config::Builder::from(&base)
            .force_path_style(true)
            .build();

        Self {
            client: Client::from_conf(config),
            bucket,
        }
    }

    /// Upload `bytes` under `key`. `content_type` is optional, e.g. "image/png".
    pub async fn upload(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<(), StorageError> {
        let mut req = self
            .client
            .put_object()
            .bucket(self.bucket.clone())
            .key(key)
            .body(ByteStream::from(bytes));
        if let Some(ct) = content_type {
            req = req.content_type(ct);
        }
        req.send()
            .await
            .map_err(|e| StorageError::Sdk(e.to_string()))?;
        Ok(())
    }

    /// Download the object at `key` as raw bytes.
    pub async fn download(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        let resp = self
            .client
            .get_object()
            .bucket(self.bucket.clone())
            .key(key)
            .send()
            .await
            .map_err(|e| StorageError::Sdk(e.to_string()))?;
        let data = resp
            .body
            .collect()
            .await
            .map_err(|e| StorageError::Sdk(e.to_string()))?;
        Ok(data.into_bytes().to_vec())
    }

    /// Check whether an object exists under `key`, without downloading it.
    pub async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        match self
            .client
            .head_object()
            .bucket(self.bucket.clone())
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                // "Not found" is a normal answer here, not an error. Any
                // other failure (connection refused, auth, ...) is real.
                if e.as_service_error().map(|se| se.is_not_found()) == Some(true) {
                    Ok(false)
                } else {
                    Err(StorageError::Sdk(e.to_string()))
                }
            }
        }
    }

    /// List object keys, optionally restricted to those starting with `prefix`.
    pub async fn list(&self, prefix: Option<&str>) -> Result<Vec<String>, StorageError> {
        let mut keys = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let mut req = self.client.list_objects_v2().bucket(self.bucket.clone());
            if let Some(p) = prefix {
                req = req.prefix(p);
            }
            if let Some(token) = continuation_token.as_deref() {
                req = req.continuation_token(token);
            }

            let resp = req
                .send()
                .await
                .map_err(|e| StorageError::Sdk(e.to_string()))?;

            keys.extend(
                resp.contents()
                    .iter()
                    .filter_map(|o| o.key().map(String::from)),
            );

            if resp.is_truncated().unwrap_or(false) {
                continuation_token = resp.next_continuation_token().map(str::to_owned);
                if continuation_token.is_none() {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(keys)
    }
}
