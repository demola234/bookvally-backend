use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::TtsRepository;
use crate::domain::TextChunk;

pub struct GetTextChunks<R> {
    pub repository: R,
}

impl<R: TtsRepository> GetTextChunks<R> {
    pub async fn execute(
        &self,
        book_file_id: Uuid,
        chapter: Option<i32>,
    ) -> Result<Vec<TextChunk>, AppError> {
        self.repository
            .get_text_chunks(book_file_id, chapter)
            .await
            .map_err(AppError::internal)
    }
}
