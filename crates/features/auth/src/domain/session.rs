use chrono::{DateTime, Utc};
use kernel::UserId;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    pub id: Uuid,
    #[sqlx(try_from = "Uuid")]
    pub user_id: UserId,
    pub device_id: Option<Uuid>,
    pub refresh_token_hash: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub handle: Option<String>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl Session {
    pub fn is_valid(&self) -> bool {
        self.revoked_at.is_none() && self.expires_at > Utc::now()
    }

    pub fn revoke(&mut self) {
        self.revoked_at = Some(Utc::now());
    }
}
