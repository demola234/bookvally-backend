use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::ReaderRepository;
use crate::domain::Highlight;

pub struct CreateHighlightInput {
    pub color: String,
    pub locator_start: String,
    pub locator_end: String,
    pub selected_text: Option<String>,
}

pub struct CreateHighlight<R> {
    pub repository: R,
}

impl<R: ReaderRepository> CreateHighlight<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        input: CreateHighlightInput,
    ) -> Result<Uuid, AppError> {
        self.repository
            .find_library_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        let highlight = Highlight::new(
            item_id,
            user_id,
            input.color,
            input.locator_start,
            input.locator_end,
            input.selected_text,
        );

        let id = self
            .repository
            .create_highlight(&highlight)
            .await
            .map_err(AppError::internal)?;

        Ok(id)
    }
}
