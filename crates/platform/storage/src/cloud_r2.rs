use async_trait::async_trait;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use serde::{Deserialize, Serialize};

use crate::port::{PresignedUpload, StorageError, StorageService};

#[derive(Debug, Clone)]
pub struct CloudR2Storage {
    pub client:     Client,
    pub bucket:     String,
    pub public_url: Option<String>,
}

impl CloudR2Storage {
    pub async fn new(config: &CloudR2StorageConfig) -> anyhow::Result<Self> {
        let sdk_config = aws_config::ConfigLoader::default()
            .endpoint_url(&config.endpoint)
            .region(aws_sdk_s3::config::Region::new(config.region.clone()))
            .credentials_provider(aws_sdk_s3::config::Credentials::new(
                &config.access_key_id,
                &config.secret_access_key,
                None,
                None,
                "R2",
            ))
            .load()
            .await;

        let client = Client::new(&sdk_config);
        Ok(Self {
            client,
            bucket:     config.bucket.clone(),
            public_url: config.public_url.clone(),
        })
    }

    fn cdn_url(&self, key: &str) -> String {
        match &self.public_url {
            Some(base) => format!("{}/{}/{}", base.trim_end_matches('/'), self.bucket, key),
            None       => format!("https://{}.r2.dev/{}", self.bucket, key),
        }
    }
}

#[async_trait]
impl StorageService for CloudR2Storage {
    async fn upload(&self, key: &str, bytes: Vec<u8>, content_type: &str) -> Result<String, StorageError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .body(ByteStream::from(bytes))
            .send()
            .await
            .map_err(|e| StorageError::UploadError(e.to_string()))?;

        Ok(self.cdn_url(key))
    }

    async fn presign_upload(&self, key: &str, content_type: &str, ttl_secs: u32) -> Result<PresignedUpload, StorageError> {
        use aws_sdk_s3::presigning::PresigningConfig;
        use std::time::Duration;

        let presigned = self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .presigned(
                PresigningConfig::expires_in(Duration::from_secs(ttl_secs as u64))
                    .map_err(|e| StorageError::PresignUploadError(e.to_string()))?,
            )
            .await
            .map_err(|e| StorageError::PresignUploadError(e.to_string()))?;

        Ok(PresignedUpload {
            upload_url: presigned.uri().to_string(),
            cdn_url:    self.cdn_url(key),
        })
    }

    async fn download(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        let resp = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| StorageError::NotFound(e.to_string()))?;

        let bytes = resp
            .body
            .collect()
            .await
            .map_err(|e| StorageError::InternalError(e.to_string()))?;

        Ok(bytes.into_bytes().to_vec())
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| StorageError::DeleteError(e.to_string()))?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudR2StorageConfig {
    pub endpoint:          String,
    pub bucket:            String,
    pub region:            String,
    pub access_key_id:     String,
    pub secret_access_key: String,
    pub public_url:        Option<String>,
}
