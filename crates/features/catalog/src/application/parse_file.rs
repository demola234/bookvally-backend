use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use kernel::AppError;

use crate::adapters::parser::BookFileParser;
use crate::application::ports::{CatalogRepository, FileParser};
use crate::domain::book::Book;
use crate::domain::book_file::ImportStatus;

pub struct ParseFile<R> {
    pub repository: R,
    pub parser:     Arc<BookFileParser>,
}

impl<R: CatalogRepository> ParseFile<R> {
    pub async fn execute(&self, file_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let mut file = self.repository
            .find_book_file(file_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("book file"))?;

        let storage_key = file.storage_key.as_deref()
            .ok_or_else(|| AppError::UnprocessableEntity("file has no storage key".into()))?;

        let parsed = self.parser
            .parse(storage_key, file.format.clone())
            .await
            .map_err(AppError::internal)?;

        // create or update book metadata
        let book_id = match file.book_id {
            Some(id) => {
                if let Some(mut book) = self.repository.find_book(&id).await.map_err(AppError::internal)? {
                    if let Some(t) = parsed.title    { book.title  = t; }
                    if let Some(a) = parsed.author   { book.author = Some(a); }
                    if let Some(p) = parsed.page_count { book.total_pages = Some(p); }
                    if let Some(l) = parsed.language { book.metadata_source = Some(l); }
                    self.repository.update_book(&book).await.map_err(AppError::internal)?;
                }
                id
            }
            None => {
                let mut book = Book::new(
                    parsed.title.unwrap_or_else(|| file.file_name.clone())
                );
                book.author     = parsed.author;
                book.total_pages = parsed.page_count;
                let id = self.repository.create_book(&book).await.map_err(AppError::internal)?;
                id
            }
        };

        file.book_id        = Some(book_id);
        file.import_status  = ImportStatus::Completed;
        file.import_progress = Some(100);
        file.imported_at    = Some(Utc::now());

        self.repository
            .update_book_file(&file)
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
