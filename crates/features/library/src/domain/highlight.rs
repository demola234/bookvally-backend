use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    pub id: Uuid,
    pub library_item_id: Uuid,
    pub user_id: Uuid,
    pub color: String,
    pub locator_start: String,
    pub locator_end: String,
    pub selected_text: Option<String>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Highlight {
    pub fn new(
        library_item_id: Uuid,
        user_id: Uuid,
        color: String,
        locator_start: String,
        locator_end: String,
        selected_text: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            library_item_id,
            user_id,
            color,
            locator_start,
            locator_end,
            selected_text,
            note: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_note(&mut self, note: Option<String>) {
        self.note = note;
        self.updated_at = Utc::now();
    }

    pub fn update_color(&mut self, color: String) {
        self.color = color;
        self.updated_at = Utc::now();
    }
}
