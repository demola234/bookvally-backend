use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::book::Book;
use crate::domain::book_file::{BookFile, ImportStatus};

// Requests

#[derive(Deserialize, ToSchema)]
pub struct ImportBookRequest {
    pub source_url:     String,
    pub file_name:      String,
    pub cloud_provider: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateBookRequest {
    pub title:            Option<String>,
    pub author:           Option<String>,
    pub published_year:   Option<i16>,
    pub isbn:             Option<String>,
    pub genre:            Option<String>,
    pub synopsis:         Option<String>,
    pub cover_url:        Option<String>,
    pub is_public_domain: Option<bool>,
}

// Responses

#[derive(Serialize, ToSchema)]
pub struct BookResponse {
    pub id:               Uuid,
    pub title:            String,
    pub author:           Option<String>,
    pub published_year:   Option<i16>,
    pub isbn:             Option<String>,
    pub genre:            Option<String>,
    pub synopsis:         Option<String>,
    pub total_pages:      Option<i32>,
    pub cover_url:        Option<String>,
    pub is_public_domain: bool,
    pub metadata_source:  Option<String>,
    pub created_at:       DateTime<Utc>,
}

impl From<Book> for BookResponse {
    fn from(b: Book) -> Self {
        Self {
            id:               b.id,
            title:            b.title,
            author:           b.author,
            published_year:   b.published_year,
            isbn:             b.isbn,
            genre:            b.genre,
            synopsis:         b.synopsis,
            total_pages:      b.total_pages,
            cover_url:        b.cover_url,
            is_public_domain: b.is_public_domain,
            metadata_source:  b.metadata_source,
            created_at:       b.created_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct BookFileResponse {
    pub id:              Uuid,
    pub user_id:         Uuid,
    pub book_id:         Option<Uuid>,
    pub file_name:       String,
    pub format:          String,
    pub size_bytes:      Option<i64>,
    pub import_status:   String,
    pub import_progress: Option<i16>,
    pub imported_at:     Option<DateTime<Utc>>,
    pub created_at:      DateTime<Utc>,
}

impl From<BookFile> for BookFileResponse {
    fn from(f: BookFile) -> Self {
        let import_status = match f.import_status {
            ImportStatus::Pending   => "pending",
            ImportStatus::Importing => "importing",
            ImportStatus::Completed => "completed",
            ImportStatus::Failed    => "failed",
        }.to_string();

        Self {
            id:              f.id,
            user_id:         f.user_id,
            book_id:         f.book_id,
            file_name:       f.file_name,
            format:          format!("{:?}", f.format).to_lowercase(),
            size_bytes:      f.size_bytes,
            import_status,
            import_progress: f.import_progress,
            imported_at:     f.imported_at,
            created_at:      f.created_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ImportBookResponse {
    pub file_id: Uuid,
    pub status:  String,
    pub message: String,
}
