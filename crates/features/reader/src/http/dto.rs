use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProgressResponse {
    pub library_item_id: Uuid,
    pub current_page: i32,
    pub progress_pct: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SaveProgressRequest {
    pub library_item_id: Uuid,
    pub current_page: i32,
    pub current_locator: Option<String>,
}
