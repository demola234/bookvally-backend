use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum HighlightError {
    #[error("Invalid locator")]
    InvalidLocator,
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
        Self {
            id: Uuid::new_v4(),
            library_item_id,
            user_id,
            color,
            locator_start,
            locator_end,
            selected_text,
            note: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
