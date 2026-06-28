use chrono::{DateTime, Utc};
use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::LibraryRepository;
use crate::domain::{ReadingSession, SessionMode};

pub struct LogSessionInput {
    pub mode: SessionMode,
    pub voice_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub pages_read: i32,
    pub minutes: f64,
}

pub struct LogSession<R> {
    pub repository: R,
}

impl<R: LibraryRepository> LogSession<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        input: LogSessionInput,
    ) -> Result<Uuid, AppError> {
        let mut item = self
            .repository
            .find_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        let session = ReadingSession::new(
            user_id,
            item_id,
            input.mode,
            input.voice_id,
            input.started_at,
            input.ended_at,
            input.pages_read,
            input.minutes,
        );

        session
            .validate()
            .map_err(|e| AppError::UnprocessableEntity(e.to_string()))?;

        let session_id = self
            .repository
            .create_session(&session)
            .await
            .map_err(AppError::internal)?;

        item.touch_last_read();
        self.repository
            .update_item(&item)
            .await
            .map_err(AppError::internal)?;

        Ok(session_id)
    }
}
