use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::TtsRepository;
use crate::domain::Voice;

pub struct GetVoice<R> {
    pub repository: R,
}

impl<R: TtsRepository> GetVoice<R> {
    pub async fn execute(&self, voice_id: Uuid) -> Result<Voice, AppError> {
        self.repository
            .find_voice(voice_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("voice"))
    }
}
