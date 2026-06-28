use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// Requests

#[derive(Debug, Deserialize, ToSchema)]
pub struct SaveProgressRequest {
    pub current_page: i32,
    pub current_locator: Option<String>,
    pub progress_pct: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EndSessionRequest {
    pub mode: String,
    pub voice_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub pages_read: i32,
    pub minutes: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateHighlightRequest {
    pub color: String,
    pub locator_start: String,
    pub locator_end: String,
    pub selected_text: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddBookmarkRequest {
    pub locator: String,
    pub page: Option<i32>,
    pub label: Option<String>,
}

// Responses

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedResponse {
    pub id: Uuid,
}

impl CreatedResponse {
    pub fn new(id: Uuid) -> Self {
        Self { id }
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
