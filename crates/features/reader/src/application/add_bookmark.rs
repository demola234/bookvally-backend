use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::ReaderRepository;
use crate::domain::Bookmark;

pub struct AddBookmarkInput {
    pub locator: String,
    pub page: Option<i32>,
    pub label: Option<String>,
}

pub struct AddBookmark<R> {
    pub repository: R,
}

impl<R: ReaderRepository> AddBookmark<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        input: AddBookmarkInput,
    ) -> Result<Uuid, AppError> {
        self.repository
            .find_library_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        let bookmark = Bookmark::new(item_id, input.locator, input.page, input.label)
            .map_err(|e| AppError::UnprocessableEntity(e.to_string()))?;

        let id = self
            .repository
            .create_bookmark(&bookmark)
            .await
            .map_err(AppError::internal)?;

        Ok(id)
    }
}
