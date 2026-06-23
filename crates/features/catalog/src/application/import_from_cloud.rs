use chrono::Utc;
use kernel::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::cloud_import::CloudImporter;
use crate::application::ports::CatalogRepository;
use crate::domain::book_file::{BookFile, BookFormat, ImportStatus};

pub struct ImportFromCloudInput {
    pub source_url: String,
    pub file_name: String,
    pub cloud_provider: String,
}

pub struct ImportFromCloud<R> {
    pub repository: R,
    pub importer: Arc<CloudImporter>,
}

impl<R: CatalogRepository> ImportFromCloud<R> {
    pub async fn execute(
        &self,
        user_id: Uuid,
        input: ImportFromCloudInput,
    ) -> Result<Uuid, AppError> {
        let format = detect_format(&input.file_name).ok_or_else(|| {
            AppError::UnprocessableEntity(
                "unsupported file format — only .pdf and .epub are accepted".into(),
            )
        })?;

        let imported = self
            .importer
            .import(&input.source_url, user_id, &input.file_name)
            .await
            .map_err(AppError::internal)?;

        let file = BookFile {
            id: Uuid::new_v4(),
            user_id,
            book_id: None,
            cloud_connection_id: None,
            source: input.cloud_provider,
            file_name: input.file_name,
            format,
            size_bytes: Some(imported.size_bytes),
            storage_key: Some(imported.storage_key),
            import_status: ImportStatus::Pending,
            import_progress: Some(0),
            imported_at: None,
            created_at: Utc::now(),
        };

        let id = self
            .repository
            .create_book_file(&file)
            .await
            .map_err(AppError::internal)?;

        Ok(id)
    }
}

fn detect_format(file_name: &str) -> Option<BookFormat> {
    let lower = file_name.to_lowercase();
    if lower.ends_with(".epub") {
        return Some(BookFormat::Epub);
    }
    if lower.ends_with(".pdf") {
        return Some(BookFormat::Pdf);
    }
    None
}
