use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::ReaderRepository;

pub struct SaveProgressInput {
    pub current_page: i32,
    pub current_locator: Option<String>,
    pub progress_pct: f64,
}

pub struct SaveProgress<R> {
    pub repository: R,
}

impl<R: ReaderRepository> SaveProgress<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        input: SaveProgressInput,
    ) -> Result<(), AppError> {
        self.repository
            .find_library_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        let pct = input.progress_pct.clamp(0.0, 100.0);

        self.repository
            .update_progress(
                item_id,
                user_id,
                input.current_page,
                input.current_locator,
                pct,
            )
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
