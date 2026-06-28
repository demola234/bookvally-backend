use async_trait::async_trait;
use chrono::{DateTime, Utc};
use persistence::PgPool;
use uuid::Uuid;

use crate::application::ports::{LibraryRepository, Page, Pagination};
use crate::domain::read_status::{AddedVia, LibraryStatus};
use crate::domain::reading_session::SessionMode;
use crate::domain::{Bookmark, Highlight, LibraryItem, ReadingSession};

#[derive(sqlx::FromRow)]
struct LibraryItemRow {
    id: Uuid,
    user_id: Uuid,
    book_id: Uuid,
    book_file_id: Option<Uuid>,
    status: String,
    current_page: i32,
    current_locator: Option<String>,
    progress_pct: f64,
    rating: Option<i16>,
    added_via: String,
    started_at: Option<DateTime<Utc>>,
    finished_at: Option<DateTime<Utc>>,
    last_read_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl From<LibraryItemRow> for LibraryItem {
    fn from(r: LibraryItemRow) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            book_id: r.book_id,
            book_file_id: r.book_file_id,
            status: LibraryStatus::try_from(r.status).unwrap_or(LibraryStatus::Queued),
            current_page: r.current_page,
            current_locator: r.current_locator,
            progress_pct: r.progress_pct,
            rating: r.rating,
            added_via: AddedVia::try_from(r.added_via).unwrap_or(AddedVia::Import),
            started_at: r.started_at,
            finished_at: r.finished_at,
            last_read_at: r.last_read_at,
            created_at: r.created_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct ReadingSessionRow {
    id: Uuid,
    user_id: Uuid,
    library_item_id: Uuid,
    mode: String,
    voice_id: Option<Uuid>,
    started_at: DateTime<Utc>,
    ended_at: Option<DateTime<Utc>>,
    pages_read: i32,
    minutes: f64,
    created_at: DateTime<Utc>,
}

impl From<ReadingSessionRow> for ReadingSession {
    fn from(r: ReadingSessionRow) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            library_item_id: r.library_item_id,
            mode: SessionMode::try_from(r.mode).unwrap_or(SessionMode::Text),
            voice_id: r.voice_id,
            started_at: r.started_at,
            ended_at: r.ended_at,
            pages_read: r.pages_read,
            minutes: r.minutes,
            created_at: r.created_at,
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

const ITEM_COLS: &str = "id, user_id, book_id, book_file_id,
    status::text, current_page, current_locator,
    progress_pct::float8, rating, added_via::text,
    started_at, finished_at, last_read_at, created_at";

const SESSION_COLS: &str = "id, user_id, library_item_id, mode::text,
    voice_id, started_at, ended_at, pages_read, minutes::float8, created_at";

const HIGHLIGHT_COLS: &str = "id, library_item_id, user_id, color, locator_start, locator_end,
     selected_text, note, created_at, updated_at";

const BOOKMARK_COLS: &str = "id, library_item_id, locator, page, label, created_at";

#[derive(Clone)]
pub struct PgLibraryRepository {
    pool: PgPool,
}

impl PgLibraryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LibraryRepository for PgLibraryRepository {
    async fn add_item(&self, item: &LibraryItem) -> anyhow::Result<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO library_items (
                id, user_id, book_id, book_file_id,
                status, current_page, progress_pct, added_via
             ) VALUES ($1, $2, $3, $4, $5::library_status, $6, $7, $8::added_via)
             RETURNING id",
        )
        .bind(item.id)
        .bind(item.user_id)
        .bind(item.book_id)
        .bind(item.book_file_id)
        .bind(item.status.as_str())
        .bind(item.current_page)
        .bind(item.progress_pct)
        .bind(item.added_via.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db) = e {
                if db.constraint() == Some("uq_library_items_user_book") {
                    return anyhow::anyhow!("book already in library");
                }
            }
            anyhow::anyhow!("add_item: {e}")
        })
    }

    async fn find_item(&self, item_id: Uuid, user_id: Uuid) -> anyhow::Result<Option<LibraryItem>> {
        let row = sqlx::query_as::<_, LibraryItemRow>(&format!(
            "SELECT {ITEM_COLS} FROM library_items WHERE id = $1 AND user_id = $2"
        ))
        .bind(item_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(LibraryItem::from))
    }

    async fn find_item_by_book(
        &self,
        book_id: Uuid,
        user_id: Uuid,
    ) -> anyhow::Result<Option<LibraryItem>> {
        let row = sqlx::query_as::<_, LibraryItemRow>(&format!(
            "SELECT {ITEM_COLS} FROM library_items WHERE book_id = $1 AND user_id = $2"
        ))
        .bind(book_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(LibraryItem::from))
    }

    async fn list_items(
        &self,
        user_id: Uuid,
        status: Option<LibraryStatus>,
        pagination: &Pagination,
    ) -> anyhow::Result<Page<LibraryItem>> {
        let status_str = status.as_ref().map(|s| s.as_str());

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM library_items
             WHERE user_id = $1
               AND ($2::library_status IS NULL OR status = $2::library_status)",
        )
        .bind(user_id)
        .bind(status_str)
        .fetch_one(&self.pool)
        .await?;

        let rows = sqlx::query_as::<_, LibraryItemRow>(&format!(
            "SELECT {ITEM_COLS} FROM library_items
             WHERE user_id = $1
               AND ($2::library_status IS NULL OR status = $2::library_status)
             ORDER BY last_read_at DESC NULLS LAST, created_at DESC
             LIMIT $3 OFFSET $4"
        ))
        .bind(user_id)
        .bind(status_str)
        .bind(pagination.limit)
        .bind(pagination.offset())
        .fetch_all(&self.pool)
        .await?;

        Ok(Page::new(
            rows.into_iter().map(LibraryItem::from).collect(),
            total,
            pagination,
        ))
    }

    async fn update_item(&self, item: &LibraryItem) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE library_items SET
                status          = $1::library_status,
                current_page    = $2,
                current_locator = $3,
                progress_pct    = $4,
                rating          = $5,
                started_at      = $6,
                finished_at     = $7,
                last_read_at    = $8
             WHERE id = $9 AND user_id = $10",
        )
        .bind(item.status.as_str())
        .bind(item.current_page)
        .bind(&item.current_locator)
        .bind(item.progress_pct)
        .bind(item.rating)
        .bind(item.started_at)
        .bind(item.finished_at)
        .bind(item.last_read_at)
        .bind(item.id)
        .bind(item.user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_item(&self, item_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM library_items WHERE id = $1 AND user_id = $2")
            .bind(item_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn create_session(&self, session: &ReadingSession) -> anyhow::Result<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO reading_sessions (
                id, user_id, library_item_id, mode, voice_id,
                started_at, ended_at, pages_read, minutes
             ) VALUES ($1, $2, $3, $4::session_mode, $5, $6, $7, $8, $9)
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
        .await
        .map_err(|e| anyhow::anyhow!("create_session: {e}"))
    }

    async fn list_sessions(
        &self,
        library_item_id: Uuid,
        user_id: Uuid,
        pagination: &Pagination,
    ) -> anyhow::Result<Page<ReadingSession>> {
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM reading_sessions rs
             JOIN library_items li ON li.id = rs.library_item_id
             WHERE rs.library_item_id = $1 AND li.user_id = $2",
        )
        .bind(library_item_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        let rows = sqlx::query_as::<_, ReadingSessionRow>(&format!(
            "SELECT rs.{SESSION_COLS}
             FROM reading_sessions rs
             JOIN library_items li ON li.id = rs.library_item_id
             WHERE rs.library_item_id = $1 AND li.user_id = $2
             ORDER BY rs.started_at DESC
             LIMIT $3 OFFSET $4"
        ))
        .bind(library_item_id)
        .bind(user_id)
        .bind(pagination.limit)
        .bind(pagination.offset())
        .fetch_all(&self.pool)
        .await?;

        Ok(Page::new(
            rows.into_iter().map(ReadingSession::from).collect(),
            total,
            pagination,
        ))
    }

    async fn create_highlight(&self, h: &Highlight) -> anyhow::Result<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO highlights (
                id, library_item_id, user_id, color,
                locator_start, locator_end, selected_text, note
             ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
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
        .await
        .map_err(|e| anyhow::anyhow!("create_highlight: {e}"))
    }

    async fn find_highlight(
        &self,
        highlight_id: Uuid,
        user_id: Uuid,
    ) -> anyhow::Result<Option<Highlight>> {
        let row = sqlx::query_as::<_, HighlightRow>(&format!(
            "SELECT {HIGHLIGHT_COLS} FROM highlights WHERE id = $1 AND user_id = $2"
        ))
        .bind(highlight_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Highlight::from))
    }

    async fn list_highlights(
        &self,
        library_item_id: Uuid,
        pagination: &Pagination,
    ) -> anyhow::Result<Page<Highlight>> {
        let total: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM highlights WHERE library_item_id = $1")
                .bind(library_item_id)
                .fetch_one(&self.pool)
                .await?;

        let rows = sqlx::query_as::<_, HighlightRow>(&format!(
            "SELECT {HIGHLIGHT_COLS} FROM highlights
             WHERE library_item_id = $1
             ORDER BY created_at ASC
             LIMIT $2 OFFSET $3"
        ))
        .bind(library_item_id)
        .bind(pagination.limit)
        .bind(pagination.offset())
        .fetch_all(&self.pool)
        .await?;

        Ok(Page::new(
            rows.into_iter().map(Highlight::from).collect(),
            total,
            pagination,
        ))
    }

    async fn update_highlight(&self, h: &Highlight) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE highlights SET note = $1, color = $2, updated_at = now()
             WHERE id = $3 AND user_id = $4",
        )
        .bind(&h.note)
        .bind(&h.color)
        .bind(h.id)
        .bind(h.user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_highlight(&self, highlight_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM highlights WHERE id = $1 AND user_id = $2")
            .bind(highlight_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // ── bookmarks ─────────────────────────────────────────────────────────────

    async fn create_bookmark(&self, b: &Bookmark) -> anyhow::Result<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
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
        .await
        .map_err(|e| anyhow::anyhow!("create_bookmark: {e}"))
    }

    async fn list_bookmarks(
        &self,
        library_item_id: Uuid,
        pagination: &Pagination,
    ) -> anyhow::Result<Page<Bookmark>> {
        let total: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM bookmarks WHERE library_item_id = $1")
                .bind(library_item_id)
                .fetch_one(&self.pool)
                .await?;

        let rows = sqlx::query_as::<_, BookmarkRow>(&format!(
            "SELECT {BOOKMARK_COLS} FROM bookmarks
             WHERE library_item_id = $1
             ORDER BY page ASC NULLS LAST, created_at ASC
             LIMIT $2 OFFSET $3"
        ))
        .bind(library_item_id)
        .bind(pagination.limit)
        .bind(pagination.offset())
        .fetch_all(&self.pool)
        .await?;

        Ok(Page::new(
            rows.into_iter().map(Bookmark::from).collect(),
            total,
            pagination,
        ))
    }

    async fn delete_bookmark(
        &self,
        bookmark_id: Uuid,
        library_item_id: Uuid,
    ) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM bookmarks WHERE id = $1 AND library_item_id = $2")
            .bind(bookmark_id)
            .bind(library_item_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
