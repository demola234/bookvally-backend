use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoiceTier {
    Basic,
    Premium,
}

impl VoiceTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Premium => "premium",
        }
    }
}

impl TryFrom<String> for VoiceTier {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "basic" => Ok(Self::Basic),
            "premium" => Ok(Self::Premium),
            other => Err(anyhow::anyhow!("unknown voice tier: {other}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Voice {
    pub id: Uuid,
    pub name: String,
    pub locale: String,
    pub tier: VoiceTier,
    pub descriptor: Option<String>,
    pub avatar_hue: Option<i16>,
    pub preview_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
