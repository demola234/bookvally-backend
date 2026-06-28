use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::application::ports::Page;
use crate::domain::{Bookmark, Highlight, LibraryItem, LibraryStatus, QueueItem, ReadingSession};

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddToShelfRequest {
    pub book_id: Uuid,
    pub book_file_id: Option<Uuid>,

    #[serde(default = "default_added_via")]
    pub added_via: String,
}

fn default_added_via() -> String {
    "import".into()
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateItemRequest {
    pub status: Option<String>,
    pub current_page: Option<i32>,
    pub current_locator: Option<String>,
    pub progress_pct: Option<f64>,
    pub rating: Option<i16>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LogSessionRequest {
    pub mode: String,
    pub voice_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub pages_read: i32,
    pub minutes: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateHighlightRequest {
    pub color: String,
    pub locator_start: String,
    pub locator_end: String,
    pub selected_text: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateHighlightRequest {
    pub color: Option<String>,

    pub note: Option<String>,
    #[serde(default)]
    pub clear_note: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBookmarkRequest {
    pub locator: String,
    pub page: Option<i32>,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LibraryItemResponse {
    pub id: Uuid,
    pub book_id: Uuid,
    pub book_file_id: Option<Uuid>,
    pub status: String,
    pub current_page: i32,
    pub current_locator: Option<String>,
    pub progress_pct: f64,
    pub rating: Option<i16>,
    pub added_via: String,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub last_read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<LibraryItem> for LibraryItemResponse {
    fn from(i: LibraryItem) -> Self {
        Self {
            id: i.id,
            book_id: i.book_id,
            book_file_id: i.book_file_id,
            status: i.status.as_str().to_string(),
            current_page: i.current_page,
            current_locator: i.current_locator,
            progress_pct: i.progress_pct,
            rating: i.rating,
            added_via: i.added_via.as_str().to_string(),
            started_at: i.started_at,
            finished_at: i.finished_at,
            last_read_at: i.last_read_at,
            created_at: i.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QueueItemResponse {
    pub id: Uuid,
    pub book_id: Uuid,
    pub book_file_id: Option<Uuid>,
    pub added_at: DateTime<Utc>,
}

impl From<QueueItem> for QueueItemResponse {
    fn from(q: QueueItem) -> Self {
        Self {
            id: q.id,
            book_id: q.book_id,
            book_file_id: q.book_file_id,
            added_at: q.added_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReadingSessionResponse {
    pub id: Uuid,
    pub library_item_id: Uuid,
    pub mode: String,
    pub voice_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub pages_read: i32,
    pub minutes: f64,
    pub created_at: DateTime<Utc>,
}

impl From<ReadingSession> for ReadingSessionResponse {
    fn from(s: ReadingSession) -> Self {
        Self {
            id: s.id,
            library_item_id: s.library_item_id,
            mode: s.mode.as_str().to_string(),
            voice_id: s.voice_id,
            started_at: s.started_at,
            ended_at: s.ended_at,
            pages_read: s.pages_read,
            minutes: s.minutes,
            created_at: s.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HighlightResponse {
    pub id: Uuid,
    pub library_item_id: Uuid,
    pub color: String,
    pub locator_start: String,
    pub locator_end: String,
    pub selected_text: Option<String>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Highlight> for HighlightResponse {
    fn from(h: Highlight) -> Self {
        Self {
            id: h.id,
            library_item_id: h.library_item_id,
            color: h.color,
            locator_start: h.locator_start,
            locator_end: h.locator_end,
            selected_text: h.selected_text,
            note: h.note,
            created_at: h.created_at,
            updated_at: h.updated_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BookmarkResponse {
    pub id: Uuid,
    pub library_item_id: Uuid,
    pub locator: String,
    pub page: Option<i32>,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Bookmark> for BookmarkResponse {
    fn from(b: Bookmark) -> Self {
        Self {
            id: b.id,
            library_item_id: b.library_item_id,
            locator: b.locator,
            page: b.page,
            label: b.label,
            created_at: b.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedResponse {
    pub id: Uuid,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PageResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub pages: i64,
}

impl<T: Serialize, U: Serialize + From<T>> From<Page<T>> for PageResponse<U> {
    fn from(p: Page<T>) -> Self {
        Self {
            items: p.items.into_iter().map(U::from).collect(),
            total: p.total,
            page: p.page,
            limit: p.limit,
            pages: p.pages,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListItemsQuery {
    pub status: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}
fn default_limit() -> i64 {
    20
}

impl ListItemsQuery {
    pub fn parsed_status(&self) -> anyhow::Result<Option<LibraryStatus>> {
        match self.status.as_deref() {
            None => Ok(None),
            Some(s) => LibraryStatus::try_from(s.to_string()).map(Some),
        }
    }
}
