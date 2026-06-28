use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::read_status::{AddedVia, LibraryStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub book_file_id: Option<Uuid>,
    pub status: LibraryStatus,
    pub current_page: i32,
    pub current_locator: Option<String>,
    pub progress_pct: f64,
    pub rating: Option<i16>,
    pub added_via: AddedVia,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub last_read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl LibraryItem {
    pub fn new(
        user_id: Uuid,
        book_id: Uuid,
        book_file_id: Option<Uuid>,
        added_via: AddedVia,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            book_id,
            book_file_id,
            status: LibraryStatus::Queued,
            current_page: 0,
            current_locator: None,
            progress_pct: 0.0,
            rating: None,
            added_via,
            started_at: None,
            finished_at: None,
            last_read_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn start_reading(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Utc::now());
        }
        self.status = LibraryStatus::Reading;
    }

    pub fn mark_finished(&mut self) {
        self.status = LibraryStatus::Finished;
        self.finished_at = Some(Utc::now());
        self.progress_pct = 100.0;
    }

    pub fn drop_book(&mut self) {
        self.status = LibraryStatus::Dropped;
        self.finished_at = None;
    }

    pub fn set_rating(&mut self, rating: i16) -> anyhow::Result<()> {
        if !(1..=5).contains(&rating) {
            return Err(anyhow::anyhow!("rating must be between 1 and 5"));
        }
        self.rating = Some(rating);
        Ok(())
    }

    pub fn update_progress(&mut self, page: i32, locator: Option<String>, pct: f64) {
        self.current_page = page;
        self.current_locator = locator;
        self.progress_pct = pct.clamp(0.0, 100.0);
    }

    pub fn touch_last_read(&mut self) {
        self.last_read_at = Some(Utc::now());
    }
}
