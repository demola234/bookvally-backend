use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::{LibraryRepository, Page, Pagination};
use crate::domain::Highlight;

pub struct CreateHighlightInput {
    pub color: String,
    pub locator_start: String,
    pub locator_end: String,
    pub selected_text: Option<String>,
    pub note: Option<String>,
}

pub struct UpdateHighlightInput {
    pub color: Option<String>,
    pub note: Option<Option<String>>,
}

pub struct CreateHighlight<R> {
    pub repository: R,
}

impl<R: LibraryRepository> CreateHighlight<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        input: CreateHighlightInput,
    ) -> Result<Uuid, AppError> {
        self.repository
            .find_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        let mut highlight = Highlight::new(
            item_id,
            user_id,
            input.color,
            input.locator_start,
            input.locator_end,
            input.selected_text,
        );

        if let Some(note) = input.note {
            highlight.update_note(Some(note));
        }

        self.repository
            .create_highlight(&highlight)
            .await
            .map_err(AppError::internal)
    }
}

pub struct ListHighlights<R> {
    pub repository: R,
}

impl<R: LibraryRepository> ListHighlights<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        pagination: Pagination,
    ) -> Result<Page<Highlight>, AppError> {
        self.repository
            .find_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        self.repository
            .list_highlights(item_id, &pagination)
            .await
            .map_err(AppError::internal)
    }
}

pub struct UpdateHighlight<R> {
    pub repository: R,
}

impl<R: LibraryRepository> UpdateHighlight<R> {
    pub async fn execute(
        &self,
        highlight_id: Uuid,
        user_id: Uuid,
        input: UpdateHighlightInput,
    ) -> Result<(), AppError> {
        let mut highlight = self
            .repository
            .find_highlight(highlight_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("highlight"))?;

        if let Some(color) = input.color {
            highlight.update_color(color);
        }

        if let Some(note) = input.note {
            highlight.update_note(note);
        }

        self.repository
            .update_highlight(&highlight)
            .await
            .map_err(AppError::internal)
    }
}

pub struct DeleteHighlight<R> {
    pub repository: R,
}

impl<R: LibraryRepository> DeleteHighlight<R> {
    pub async fn execute(&self, highlight_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repository
            .delete_highlight(highlight_id, user_id)
            .await
            .map_err(AppError::internal)
    }
}
