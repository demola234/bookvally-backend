use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use persistence::PgPool;

use crate::application::ports::{LibraryItemView, ReaderRepository};
use crate::domain::{Bookmark, Highlight, ReaderSession};

#[derive(Clone)]
pub struct PgReaderRepository {
    pool: PgPool,
}

impl PgReaderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct LibraryItemViewRow {
    id: Uuid,
    user_id: Uuid,
    current_page: i32,
    current_locator: Option<String>,
    progress_pct: f64,
}

impl From<LibraryItemViewRow> for LibraryItemView {
    fn from(r: LibraryItemViewRow) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            current_page: r.current_page,
            current_locator: r.current_locator,
            progress_pct: r.progress_pct,
        }
    }
}

#[derive(sqlx::FromRow)]
struct HighlightRow {
    id: Uuid,
    library_item_id: Uuid,
    user_id: Uuid,
    color: String,
    locator_start: String,
    locator_end: String,
    selected_text: Option<String>,
    note: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<HighlightRow> for Highlight {
    fn from(r: HighlightRow) -> Self {
        Self {
            id: r.id,
            library_item_id: r.library_item_id,
            user_id: r.user_id,
            color: r.color,
            locator_start: r.locator_start,
            locator_end: r.locator_end,
            selected_text: r.selected_text,
            note: r.note,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct BookmarkRow {
    id: Uuid,
    library_item_id: Uuid,
    locator: String,
    page: Option<i32>,
    label: Option<String>,
    created_at: DateTime<Utc>,
}

impl From<BookmarkRow> for Bookmark {
    fn from(r: BookmarkRow) -> Self {
        Self {
            id: r.id,
            library_item_id: r.library_item_id,
            locator: r.locator,
            page: r.page,
            label: r.label,
            created_at: r.created_at,
        }
    }
}

#[async_trait]
impl ReaderRepository for PgReaderRepository {
    async fn find_library_item(
        &self,
        item_id: Uuid,
        user_id: Uuid,
    ) -> anyhow::Result<Option<LibraryItemView>> {
        let row = sqlx::query_as::<_, LibraryItemViewRow>(
            "SELECT id, user_id, current_page, current_locator, progress_pct::float8
             FROM library_items WHERE id = $1 AND user_id = $2",
        )
        .bind(item_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(LibraryItemView::from))
    }

    async fn update_progress(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        page: i32,
        locator: Option<String>,
        pct: f64,
    ) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE library_items
             SET current_page = $1, current_locator = $2, progress_pct = $3
             WHERE id = $4 AND user_id = $5",
        )
        .bind(page)
        .bind(locator)
        .bind(pct)
        .bind(item_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_last_read(&self, item_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("UPDATE library_items SET last_read_at = NOW() WHERE id = $1 AND user_id = $2")
            .bind(item_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn log_session(&self, session: &ReaderSession) -> anyhow::Result<Uuid> {
        let id = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO reading_sessions
             (id, user_id, library_item_id, mode, voice_id, started_at, ended_at, pages_read, minutes)
             VALUES ($1, $2, $3, $4::session_mode, $5, $6, $7, $8, $9)
             RETURNING id",
        )
        .bind(session.id)
        .bind(session.user_id)
        .bind(session.library_item_id)
        .bind(session.mode.as_str())
        .bind(session.voice_id)
        .bind(session.started_at)
        .bind(session.ended_at)
        .bind(session.pages_read)
        .bind(session.minutes)
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    async fn create_highlight(&self, h: &Highlight) -> anyhow::Result<Uuid> {
        let id = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO highlights
             (id, library_item_id, user_id, color, locator_start, locator_end, selected_text, note)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id",
        )
        .bind(h.id)
        .bind(h.library_item_id)
        .bind(h.user_id)
        .bind(&h.color)
        .bind(&h.locator_start)
        .bind(&h.locator_end)
        .bind(&h.selected_text)
        .bind(&h.note)
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    async fn list_highlights(&self, library_item_id: Uuid) -> anyhow::Result<Vec<Highlight>> {
        let rows = sqlx::query_as::<_, HighlightRow>(
            "SELECT id, library_item_id, user_id, color, locator_start, locator_end,
                    selected_text, note, created_at, updated_at
             FROM highlights WHERE library_item_id = $1
             ORDER BY created_at ASC",
        )
        .bind(library_item_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Highlight::from).collect())
    }

    async fn delete_highlight(&self, highlight_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM highlights WHERE id = $1 AND user_id = $2")
            .bind(highlight_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn create_bookmark(&self, b: &Bookmark) -> anyhow::Result<Uuid> {
        let id = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO bookmarks (id, library_item_id, locator, page, label)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id",
        )
        .bind(b.id)
        .bind(b.library_item_id)
        .bind(&b.locator)
        .bind(b.page)
        .bind(&b.label)
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }
}
