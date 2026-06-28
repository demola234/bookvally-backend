use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use persistence::PgPool;

use crate::application::ports::TtsRepository;
use crate::domain::{TextChunk, Voice, VoiceTier};

#[derive(Clone)]
pub struct PgTtsRepository {
    pool: PgPool,
}

impl PgTtsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct VoiceRow {
    id: Uuid,
    name: String,
    locale: String,
    tier: String,
    descriptor: Option<String>,
    avatar_hue: Option<i16>,
    preview_url: Option<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
}

impl TryFrom<VoiceRow> for Voice {
    type Error = anyhow::Error;

    fn try_from(r: VoiceRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: r.id,
            name: r.name,
            locale: r.locale,
            tier: VoiceTier::try_from(r.tier)?,
            descriptor: r.descriptor,
            avatar_hue: r.avatar_hue,
            preview_url: r.preview_url,
            is_active: r.is_active,
            created_at: r.created_at,
        })
    }
}

#[derive(sqlx::FromRow)]
struct ChunkRow {
    chapter: i32,
    sequence: i32,
    text: String,
    char_count: i32,
}

#[async_trait]
impl TtsRepository for PgTtsRepository {
    async fn find_voice(&self, id: Uuid) -> anyhow::Result<Option<Voice>> {
        let row = sqlx::query_as::<_, VoiceRow>(
            "SELECT id, name, locale, tier::text, descriptor, avatar_hue, preview_url,
                    is_active, created_at
             FROM voices WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(Voice::try_from).transpose()
    }

    async fn list_voices(
        &self,
        locale: Option<String>,
        tier: Option<VoiceTier>,
    ) -> anyhow::Result<Vec<Voice>> {
        let rows = sqlx::query_as::<_, VoiceRow>(
            "SELECT id, name, locale, tier::text, descriptor, avatar_hue, preview_url,
                    is_active, created_at
             FROM voices
             WHERE is_active = true
               AND ($1::text IS NULL OR locale = $1)
               AND ($2::text IS NULL OR tier = $2::voice_tier)
             ORDER BY name ASC",
        )
        .bind(locale)
        .bind(tier.as_ref().map(|t| t.as_str()))
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(Voice::try_from).collect()
    }

    async fn get_text_chunks(
        &self,
        book_file_id: Uuid,
        chapter: Option<i32>,
    ) -> anyhow::Result<Vec<TextChunk>> {
        let rows = if let Some(ch) = chapter {
            sqlx::query_as::<_, ChunkRow>(
                "SELECT chapter, sequence, text, char_count
                 FROM book_text_chunks
                 WHERE book_file_id = $1 AND chapter = $2
                 ORDER BY sequence ASC",
            )
            .bind(book_file_id)
            .bind(ch)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, ChunkRow>(
                "SELECT chapter, sequence, text, char_count
                 FROM book_text_chunks
                 WHERE book_file_id = $1
                 ORDER BY chapter ASC, sequence ASC",
            )
            .bind(book_file_id)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows
            .into_iter()
            .map(|r| TextChunk {
                book_file_id,
                chapter: r.chapter,
                sequence: r.sequence,
                text: r.text,
                char_count: r.char_count,
            })
            .collect())
    }
}
