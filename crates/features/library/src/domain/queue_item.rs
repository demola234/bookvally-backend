use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::shelf_entry::LibraryItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: Uuid,
    pub book_id: Uuid,
    pub book_file_id: Option<Uuid>,
    pub added_at: DateTime<Utc>,
}

impl From<&LibraryItem> for QueueItem {
    fn from(item: &LibraryItem) -> Self {
        Self {
            id: item.id,
            book_id: item.book_id,
            book_file_id: item.book_file_id,
            added_at: item.created_at,
        }
    }
}
