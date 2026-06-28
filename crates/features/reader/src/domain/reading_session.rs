use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionMode {
    Text,
    Tts,
}

impl SessionMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Tts => "tts",
        }
    }
}

impl TryFrom<&str> for SessionMode {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "text" => Ok(Self::Text),
            "tts" => Ok(Self::Tts),
            other => Err(anyhow::anyhow!("unknown session mode: {other}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReaderSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub library_item_id: Uuid,
    pub mode: SessionMode,
    pub voice_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub pages_read: i32,
    pub minutes: f64,
}

impl ReaderSession {
    pub fn new(
        user_id: Uuid,
        library_item_id: Uuid,
        mode: SessionMode,
        voice_id: Option<Uuid>,
        started_at: DateTime<Utc>,
        ended_at: DateTime<Utc>,
        pages_read: i32,
        minutes: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            library_item_id,
            mode,
            voice_id,
            started_at,
            ended_at,
            pages_read,
            minutes,
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.ended_at <= self.started_at {
            return Err(anyhow::anyhow!("ended_at must be after started_at"));
        }
        if self.pages_read < 0 {
            return Err(anyhow::anyhow!("pages_read cannot be negative"));
        }
        if self.minutes < 0.0 {
            return Err(anyhow::anyhow!("minutes cannot be negative"));
        }
        Ok(())
    }
}
