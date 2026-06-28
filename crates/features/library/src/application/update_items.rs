use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::LibraryRepository;
use crate::domain::shelf_entry::LibraryItem;
use crate::domain::LibraryStatus;

pub struct UpdateItemInput {
    pub status: Option<LibraryStatus>,
    pub current_page: Option<i32>,
    pub current_locator: Option<String>,
    pub progress_pct: Option<f64>,
    pub rating: Option<i16>,
}

pub struct UpdateItem<R> {
    pub repository: R,
}

impl<R: LibraryRepository> UpdateItem<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        input: UpdateItemInput,
    ) -> Result<LibraryItem, AppError> {
        let mut item = self
            .repository
            .find_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        if let Some(status) = input.status {
            match status {
                LibraryStatus::Reading => item.start_reading(),
                LibraryStatus::Finished => item.mark_finished(),
                LibraryStatus::Dropped => item.drop_book(),
                LibraryStatus::Queued => item.status = LibraryStatus::Queued,
            }
        }

        if let Some(rating) = input.rating {
            item.set_rating(rating)
                .map_err(|e| AppError::UnprocessableEntity(e.to_string()))?;
        }

        // progress update only applies when not finished (mark_finished locks at 100)
        if item.status != LibraryStatus::Finished {
            if input.current_page.is_some()
                || input.current_locator.is_some()
                || input.progress_pct.is_some()
            {
                item.update_progress(
                    input.current_page.unwrap_or(item.current_page),
                    input.current_locator.or(item.current_locator.clone()),
                    input.progress_pct.unwrap_or(item.progress_pct),
                );
            }
        }

        self.repository
            .update_item(&item)
            .await
            .map_err(AppError::internal)?;

        Ok(item)
    }
}
