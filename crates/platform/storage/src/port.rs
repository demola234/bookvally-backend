use async_trait::async_trait;

#[async_trait]
pub trait StorageService: Send + Sync {
    async fn upload(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: &str,
    ) -> Result<String, StorageError>;
    async fn presign_upload(
        &self,
        key: &str,
        content_type: &str,
        ttl_secs: u32,
    ) -> Result<PresignedUpload, StorageError>;
    async fn download(&self, key: &str) -> Result<Vec<u8>, StorageError>;
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
}

#[derive(Debug, Clone)]
pub struct PresignedUpload {
    pub upload_url: String,
    pub cdn_url: String,
}

#[derive(Debug, Clone)]
pub enum StorageError {
    UploadError(String),
    PresignUploadError(String),
    DeleteError(String),
    NotFound(String),
    InternalError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UploadError(s) => write!(f, "upload error: {s}"),
            Self::PresignUploadError(s) => write!(f, "presign error: {s}"),
            Self::DeleteError(s) => write!(f, "delete error: {s}"),
            Self::NotFound(s) => write!(f, "not found: {s}"),
            Self::InternalError(s) => write!(f, "internal error: {s}"),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<anyhow::Error> for StorageError {
    fn from(err: anyhow::Error) -> Self {
        Self::InternalError(err.to_string())
    }
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        Self::InternalError(err.to_string())
    }
}

impl From<aws_sdk_s3::Error> for StorageError {
    fn from(err: aws_sdk_s3::Error) -> Self {
        Self::InternalError(err.to_string())
    }
}
