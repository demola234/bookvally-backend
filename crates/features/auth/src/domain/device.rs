use chrono::{DateTime, Utc};
use kernel::UserId;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Device {
    pub id:          Uuid,
    pub user_id:     UserId,
    pub platform:    String,
    pub device_name: Option<String>,
    pub push_token:  Option<String>,
    pub app_version: Option<String>,
    pub last_seen_at: Option<DateTime<Utc>>,
}
