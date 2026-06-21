use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;
use persistence::PgPool;

use crate::application::ports::CatalogRepository;
use crate::domain::book::Book;
use crate::domain::book_file::{BookFile, BookFormat, ImportStatus};

#[derive(Clone)]
pub struct PgCatalogRepository {
    pool: PgPool,
}

impl PgCatalogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


#[derive(sqlx::FromRow)]
struct BookRow {
    id:               Uuid,
    title:            String,
    author:           Option<String>,
    published_year:   Option<i16>,
    isbn:             Option<String>,
    genre:            Option<String>,
    synopsis:         Option<String>,
    total_pages:      Option<i32>,
    cover_url:        Option<String>,
    is_public_domain: bool,
    metadata_source:  Option<String>,
    created_at:       DateTime<Utc>,
}

impl From<BookRow> for Book {
    fn from(r: BookRow) -> Self {
        Self {
            id:               r.id,
            title:            r.title,
            author:           r.author,
            published_year:   r.published_year,
            isbn:             r.isbn,
            genre:            r.genre,
            synopsis:         r.synopsis,
            total_pages:      r.total_pages,
            cover_url:        r.cover_url,
            is_public_domain: r.is_public_domain,
            metadata_source:  r.metadata_source,
            created_at:       r.created_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct BookFileRow {
    id:                  Uuid,
    user_id:             Uuid,
    book_id:             Option<Uuid>,
    cloud_connection_id: Option<Uuid>,
    source:              String,
    file_name:           String,
    format:              String,
    size_bytes:          Option<i64>,
    storage_key:         Option<String>,
    import_status:       String,
    import_progress:     Option<i16>,
    imported_at:         Option<DateTime<Utc>>,
    created_at:          DateTime<Utc>,
}

impl From<BookFileRow> for BookFile {
    fn from(r: BookFileRow) -> Self {
        Self {
            id:                  r.id,
            user_id:             r.user_id,
            book_id:             r.book_id,
            cloud_connection_id: r.cloud_connection_id,
            source:              r.source,
            file_name:           r.file_name,
            format:              BookFormat::try_from(r.format).unwrap_or(BookFormat::Pdf),
            size_bytes:          r.size_bytes,
            storage_key:         r.storage_key,
            import_status:       ImportStatus::try_from(r.import_status).unwrap_or(ImportStatus::Pending),
            import_progress:     r.import_progress,
            imported_at:         r.imported_at,
            created_at:          r.created_at,
        }
    }
}

const SELECT_COLS: &str =
    "id, title, author, published_year, isbn, genre,
     synopsis, total_pages, cover_url, is_public_domain,
     metadata_source, created_at";


#[async_trait]
impl CatalogRepository for PgCatalogRepository {
    async fn create_book(&self, book: &Book) -> anyhow::Result<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO books (
                id, title, author, published_year, isbn, genre,
                synopsis, total_pages, cover_url, is_public_domain, metadata_source
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id",
        )
        .bind(book.id)
        .bind(&book.title)
        .bind(&book.author)
        .bind(book.published_year)
        .bind(&book.isbn)
        .bind(&book.genre)
        .bind(&book.synopsis)
        .bind(book.total_pages)
        .bind(&book.cover_url)
        .bind(book.is_public_domain)
        .bind(&book.metadata_source)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("create_book: {e}"))
    }

    async fn find_book(&self, book_id: &Uuid) -> anyhow::Result<Option<Book>> {
        let row = sqlx::query_as::<_, BookRow>(
            &format!("SELECT {SELECT_COLS} FROM books WHERE id = $1"),
        )
        .bind(book_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Book::from))
    }

    async fn list_books(&self, user_id: Uuid) -> anyhow::Result<Vec<Book>> {
        // books joined via book_files to scope to this user
        let rows = sqlx::query_as::<_, BookRow>(&format!(
            "SELECT DISTINCT b.{SELECT_COLS}
             FROM books b
             JOIN book_files bf ON bf.book_id = b.id
             WHERE bf.user_id = $1
             ORDER BY b.created_at DESC",
        ))
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Book::from).collect())
    }

    async fn update_book(&self, book: &Book) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE books SET
                title = $1, author = $2, published_year = $3, isbn = $4, genre = $5,
                synopsis = $6, total_pages = $7, cover_url = $8,
                is_public_domain = $9, metadata_source = $10
             WHERE id = $11",
        )
        .bind(&book.title)
        .bind(&book.author)
        .bind(book.published_year)
        .bind(&book.isbn)
        .bind(&book.genre)
        .bind(&book.synopsis)
        .bind(book.total_pages)
        .bind(&book.cover_url)
        .bind(book.is_public_domain)
        .bind(&book.metadata_source)
        .bind(book.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_book(&self, book_id: &Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM books WHERE id = $1")
            .bind(book_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_book_by_title(&self, title: &str) -> anyhow::Result<Option<Book>> {
        let row = sqlx::query_as::<_, BookRow>(&format!(
            "SELECT {SELECT_COLS} FROM books WHERE lower(title) = lower($1) LIMIT 1",
        ))
        .bind(title)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Book::from))
    }

    async fn find_books_by_author(&self, author: &str) -> anyhow::Result<Vec<Book>> {
        let rows = sqlx::query_as::<_, BookRow>(&format!(
            "SELECT {SELECT_COLS} FROM books WHERE lower(author) = lower($1)",
        ))
        .bind(author)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Book::from).collect())
    }

    async fn find_books_by_genre(&self, genre: &str) -> anyhow::Result<Vec<Book>> {
        let rows = sqlx::query_as::<_, BookRow>(&format!(
            "SELECT {SELECT_COLS} FROM books WHERE lower(genre) = lower($1)",
        ))
        .bind(genre)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Book::from).collect())
    }

    // ── BookFile methods ──────────────────────────────────────

    async fn create_book_file(&self, file: &BookFile) -> anyhow::Result<Uuid> {
        let format = match file.format {
            BookFormat::Pdf  => "pdf",
            BookFormat::Epub => "epub",
        };
        let status = match file.import_status {
            ImportStatus::Pending   => "pending",
            ImportStatus::Importing => "importing",
            ImportStatus::Completed => "completed",
            ImportStatus::Failed    => "failed",
        };

        sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO book_files (
                id, user_id, book_id, cloud_connection_id, source,
                file_name, format, size_bytes, storage_key,
                import_status, import_progress, imported_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7::book_format, $8, $9, $10::import_status, $11, $12)
            RETURNING id",
        )
        .bind(file.id)
        .bind(file.user_id)
        .bind(file.book_id)
        .bind(file.cloud_connection_id)
        .bind(&file.source)
        .bind(&file.file_name)
        .bind(format)
        .bind(file.size_bytes)
        .bind(&file.storage_key)
        .bind(status)
        .bind(file.import_progress)
        .bind(file.imported_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("create_book_file: {e}"))
    }

    async fn find_book_file(&self, file_id: Uuid, user_id: Uuid) -> anyhow::Result<Option<BookFile>> {
        let row = sqlx::query_as::<_, BookFileRow>(
            "SELECT id, user_id, book_id, cloud_connection_id, source::text,
                    file_name, format::text, size_bytes, storage_key,
                    import_status::text, import_progress, imported_at, created_at
             FROM book_files
             WHERE id = $1 AND user_id = $2",
        )
        .bind(file_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(BookFile::from))
    }

    async fn list_book_files(&self, user_id: Uuid) -> anyhow::Result<Vec<BookFile>> {
        let rows = sqlx::query_as::<_, BookFileRow>(
            "SELECT id, user_id, book_id, cloud_connection_id, source::text,
                    file_name, format::text, size_bytes, storage_key,
                    import_status::text, import_progress, imported_at, created_at
             FROM book_files
             WHERE user_id = $1
             ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(BookFile::from).collect())
    }

    async fn update_book_file(&self, file: &BookFile) -> anyhow::Result<()> {
        let status = match file.import_status {
            ImportStatus::Pending   => "pending",
            ImportStatus::Importing => "importing",
            ImportStatus::Completed => "completed",
            ImportStatus::Failed    => "failed",
        };

        sqlx::query(
            "UPDATE book_files SET
                book_id         = $1,
                import_status   = $2::import_status,
                import_progress = $3,
                imported_at     = $4,
                storage_key     = $5
             WHERE id = $6 AND user_id = $7",
        )
        .bind(file.book_id)
        .bind(status)
        .bind(file.import_progress)
        .bind(file.imported_at)
        .bind(&file.storage_key)
        .bind(file.id)
        .bind(file.user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_book_file(&self, file_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM book_files WHERE id = $1 AND user_id = $2")
            .bind(file_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
