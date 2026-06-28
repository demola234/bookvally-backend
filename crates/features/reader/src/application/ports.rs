use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{Bookmark, Highlight, ReaderSession};

#[derive(Debug, Clone)]
pub struct LibraryItemView {
    pub id: Uuid,
    pub user_id: Uuid,
    pub current_page: i32,
    pub current_locator: Option<String>,
    pub progress_pct: f64,
}



#[async_trait]
pub trait ReaderRepository: Send + Sync + Clone {
    async fn find_library_item(
        &self,
        item_id: Uuid,
        user_id: Uuid,
    ) -> anyhow::Result<Option<LibraryItemView>>;

    async fn update_progress(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        page: i32,
        locator: Option<String>,
        pct: f64,
    ) -> anyhow::Result<()>;

    async fn update_last_read(&self, item_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;

    async fn log_session(&self, session: &ReaderSession) -> anyhow::Result<Uuid>;

    async fn create_highlight(&self, h: &Highlight) -> anyhow::Result<Uuid>;
    async fn list_highlights(&self, library_item_id: Uuid) -> anyhow::Result<Vec<Highlight>>;
    async fn delete_highlight(&self, highlight_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;

    async fn create_bookmark(&self, b: &Bookmark) -> anyhow::Result<Uuid>;
}
