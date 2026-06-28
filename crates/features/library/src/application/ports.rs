use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{Bookmark, Highlight, LibraryItem, LibraryStatus, ReadingSession};

#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: i64,
    pub limit: i64,
}

impl Pagination {
    pub fn new(page: i64, limit: i64) -> Self {
        Self {
            page: page.max(1),
            limit: limit.clamp(1, 100),
        }
    }

    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.limit
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1, limit: 20 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub pages: i64,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, total: i64, pagination: &Pagination) -> Self {
        let pages = if pagination.limit == 0 {
            0
        } else {
            (total as f64 / pagination.limit as f64).ceil() as i64
        };
        Self {
            items,
            total,
            page: pagination.page,
            limit: pagination.limit,
            pages,
        }
    }
}

#[async_trait]
pub trait LibraryRepository: Send + Sync + Clone {
    async fn add_item(&self, item: &LibraryItem) -> anyhow::Result<Uuid>;
    async fn find_item(&self, item_id: Uuid, user_id: Uuid) -> anyhow::Result<Option<LibraryItem>>;
    async fn find_item_by_book(
        &self,
        book_id: Uuid,
        user_id: Uuid,
    ) -> anyhow::Result<Option<LibraryItem>>;
    async fn list_items(
        &self,
        user_id: Uuid,
        status: Option<LibraryStatus>,
        pagination: &Pagination,
    ) -> anyhow::Result<Page<LibraryItem>>;
    async fn update_item(&self, item: &LibraryItem) -> anyhow::Result<()>;
    async fn delete_item(&self, item_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;

    async fn create_session(&self, session: &ReadingSession) -> anyhow::Result<Uuid>;
    async fn list_sessions(
        &self,
        library_item_id: Uuid,
        user_id: Uuid,
        pagination: &Pagination,
    ) -> anyhow::Result<Page<ReadingSession>>;

    async fn create_highlight(&self, highlight: &Highlight) -> anyhow::Result<Uuid>;
    async fn find_highlight(
        &self,
        highlight_id: Uuid,
        user_id: Uuid,
    ) -> anyhow::Result<Option<Highlight>>;
    async fn list_highlights(
        &self,
        library_item_id: Uuid,
        pagination: &Pagination,
    ) -> anyhow::Result<Page<Highlight>>;
    async fn update_highlight(&self, highlight: &Highlight) -> anyhow::Result<()>;
    async fn delete_highlight(&self, highlight_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;

    async fn create_bookmark(&self, bookmark: &Bookmark) -> anyhow::Result<Uuid>;
    async fn list_bookmarks(
        &self,
        library_item_id: Uuid,
        pagination: &Pagination,
    ) -> anyhow::Result<Page<Bookmark>>;
    async fn delete_bookmark(&self, bookmark_id: Uuid, library_item_id: Uuid)
        -> anyhow::Result<()>;
}
