use kernel::AppError;

use crate::application::ports::TtsRepository;
use crate::domain::{Voice, VoiceTier};

pub struct ListVoices<R> {
    pub repository: R,
}

impl<R: TtsRepository> ListVoices<R> {
    pub async fn execute(
        &self,
        locale: Option<String>,
        tier: Option<VoiceTier>,
    ) -> Result<Vec<Voice>, AppError> {
        self.repository
            .list_voices(locale, tier)
            .await
            .map_err(AppError::internal)
    }
}
