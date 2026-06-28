use std::sync::Arc;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use kernel::AppError;
use messaging::{KafkaProducer, READING_EVENT};

use crate::application::ports::ReaderRepository;
use crate::domain::{ReaderSession, SessionMode};
use crate::events::{ReadingSessionLogged, READING_SESSION_LOGGED};

pub struct EndSessionInput {
    pub mode: SessionMode,
    pub voice_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub pages_read: i32,
    pub minutes: f64,
}

pub struct EndSession<R> {
    pub repository: R,
    pub kafka: Arc<KafkaProducer>,
}

impl<R: ReaderRepository> EndSession<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        input: EndSessionInput,
    ) -> Result<Uuid, AppError> {
        self.repository
            .find_library_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        let session = ReaderSession::new(
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
            .log_session(&session)
            .await
            .map_err(AppError::internal)?;

        self.repository
            .update_last_read(item_id, user_id)
            .await
            .map_err(AppError::internal)?;

        let event = kernel::EventEnvelope::new(
            READING_SESSION_LOGGED,
            ReadingSessionLogged {
                user_id,
                library_item_id: item_id,
                minutes: session.minutes,
                pages_read: session.pages_read,
                session_mode: session.mode.as_str().to_string(),
                logged_at: Utc::now(),
            },
        );

        if let Err(e) = self
            .kafka
            .publish(READING_EVENT, &user_id.to_string(), &event)
            .await
        {
            tracing::warn!("failed to publish reading_session.logged: {e}");
        }

        Ok(session_id)
    }
}
