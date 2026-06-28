use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

// Query params

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListVoicesQuery {
    pub locale: Option<String>,
    pub tier: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetChunksQuery {
    pub chapter: Option<i32>,
}

// Responses
#[derive(Debug, Serialize, ToSchema)]
pub struct VoiceResponse {
    pub id: Uuid,
    pub name: String,
    pub locale: String,
    pub tier: String,
    pub descriptor: Option<String>,
    pub avatar_hue: Option<i16>,
    pub preview_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TextChunkResponse {
    pub chapter: i32,
    pub sequence: i32,
    pub text: String,
    pub char_count: i32,
}
