use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{TextChunk, Voice, VoiceTier};

#[async_trait]
pub trait TtsRepository: Send + Sync + Clone {
    async fn find_voice(&self, id: Uuid) -> anyhow::Result<Option<Voice>>;
    async fn list_voices(
        &self,
        locale: Option<String>,
        tier: Option<VoiceTier>,
    ) -> anyhow::Result<Vec<Voice>>;

    async fn get_text_chunks(
        &self,
        book_file_id: Uuid,
        chapter: Option<i32>,
    ) -> anyhow::Result<Vec<TextChunk>>;
}
