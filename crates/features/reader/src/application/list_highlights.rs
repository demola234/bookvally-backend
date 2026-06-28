use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::ReaderRepository;
use crate::domain::Highlight;

pub struct ListHighlights<R> {
    pub repository: R,
}

impl<R: ReaderRepository> ListHighlights<R> {
    pub async fn execute(&self, item_id: Uuid, user_id: Uuid) -> Result<Vec<Highlight>, AppError> {
        self.repository
            .find_library_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        let highlights = self
            .repository
            .list_highlights(item_id)
            .await
            .map_err(AppError::internal)?;

        Ok(highlights)
    }
}
