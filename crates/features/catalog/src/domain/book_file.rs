use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookFormat {
    Pdf,
    Epub,
}

impl TryFrom<String> for BookFormat {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "pdf" => Ok(Self::Pdf),
            "epub" => Ok(Self::Epub),
            other => Err(format!("unknown book format: {other}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportStatus {
    Pending,
    Importing,
    Completed,
    Failed,
}

impl TryFrom<String> for ImportStatus {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "pending" => Ok(Self::Pending),
            "importing" => Ok(Self::Importing),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            other => Err(format!("unknown import status: {other}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BookFile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Option<Uuid>,
    pub cloud_connection_id: Option<Uuid>,
    pub source: String,
    pub file_name: String,
    pub format: BookFormat,
    pub size_bytes: Option<i64>,
    pub storage_key: Option<String>,
    pub import_status: ImportStatus,
    pub import_progress: Option<i16>,
    pub imported_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
