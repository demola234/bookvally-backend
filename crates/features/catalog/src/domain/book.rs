use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Shared book metadata — not user-specific.
/// Per-user file records (format, storage key, import status) live in `BookFile`.
#[derive(Debug, Clone)]
pub struct Book {
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

impl Book {
    pub fn new(title: String) -> Self {
        Self {
            id:               Uuid::new_v4(),
            title,
            author:           None,
            published_year:   None,
            isbn:             None,
            genre:            None,
            synopsis:         None,
            total_pages:      None,
            cover_url:        None,
            is_public_domain: false,
            metadata_source:  None,
            created_at:       Utc::now(),
        }
    }
}
