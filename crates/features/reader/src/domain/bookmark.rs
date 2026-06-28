use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: Uuid,
    pub library_item_id: Uuid,
    pub locator: String,
    pub page: Option<i32>,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Bookmark {
    pub fn new(
        library_item_id: Uuid,
        locator: String,
        page: Option<i32>,
        label: Option<String>,
    ) -> anyhow::Result<Self> {
        if let Some(ref l) = label {
            if l.len() > 140 {
                return Err(anyhow::anyhow!("label max 140 characters"));
            }
        }
        Ok(Self {
            id: Uuid::new_v4(),
            library_item_id,
            locator,
            page,
            label,
            created_at: Utc::now(),
        })
    }
}
