use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::book_file::ImportStatus;

#[derive(Debug, Clone)]
pub enum CloudProvider {
    GoogleDrive,
    Dropbox,
    ICloud,
    OneDrive,
}

impl TryFrom<String> for CloudProvider {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "google_drive" => Ok(Self::GoogleDrive),
            "dropbox"      => Ok(Self::Dropbox),
            "icloud"       => Ok(Self::ICloud),
            "onedrive"     => Ok(Self::OneDrive),
            other          => Err(format!("unknown cloud provider: {other}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportJob {
    pub id:             Uuid,
    pub user_id:        Uuid,
    pub source_url:     String,
    pub cloud_provider: CloudProvider,
    pub status:         ImportStatus,
    pub error_message:  Option<String>,
    pub book_id:        Option<Uuid>,
    pub created_at:     DateTime<Utc>,
    pub updated_at:     DateTime<Utc>,
}
