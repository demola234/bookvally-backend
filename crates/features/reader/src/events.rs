use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const READING_SESSION_LOGGED: &str = "reading_session.logged";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingSessionLogged {
    pub user_id: Uuid,
    pub library_item_id: Uuid,
    pub minutes: f64,
    pub pages_read: i32,
    pub session_mode: String,
    pub logged_at: DateTime<Utc>,
}
