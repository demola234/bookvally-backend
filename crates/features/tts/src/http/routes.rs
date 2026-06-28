use axum::extract::{Path, Query, State};
use axum::{routing::get, Json, Router};
use uuid::Uuid;

use auth_kit::JwtAuthExtractor;
use http_kit::{v1, HttpError};
use kernel::AppError;

use crate::application::get_playback::GetTextChunks;
use crate::application::list_voices::ListVoices;
use crate::application::synthesize::GetVoice;
use crate::domain::VoiceTier;
use crate::http::dto::{GetChunksQuery, ListVoicesQuery, TextChunkResponse, VoiceResponse};
use crate::wiring::TtsState;

pub fn routes() -> Router<TtsState> {
    v1(Router::new()
        .route("/tts/voices", get(list_voices))
        .route("/tts/voices/{id}", get(get_voice))
        .route("/tts/books/{file_id}/chunks", get(get_chunks)))
}

#[utoipa::path(
    get,
    path = "/v1/tts/voices",
    tag = "tts",
    params(ListVoicesQuery),
    responses(
        (status = 200, description = "Active voices", body = Vec<VoiceResponse>),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
async fn list_voices(
    State(state): State<TtsState>,
    _auth: JwtAuthExtractor,
    Query(q): Query<ListVoicesQuery>,
) -> Result<Json<Vec<VoiceResponse>>, HttpError> {
    let tier = q
        .tier
        .map(VoiceTier::try_from)
        .transpose()
        .map_err(|e| HttpError::from(AppError::UnprocessableEntity(e.to_string())))?;

    let voices = ListVoices {
        repository: state.repo.clone(),
    }
    .execute(q.locale, tier)
    .await
    .map_err(HttpError::from)?;

    let resp = voices
        .into_iter()
        .map(|v| VoiceResponse {
            id: v.id,
            name: v.name,
            locale: v.locale,
            tier: v.tier.as_str().to_owned(),
            descriptor: v.descriptor,
            avatar_hue: v.avatar_hue,
            preview_url: v.preview_url,
            created_at: v.created_at,
        })
        .collect();

    Ok(Json(resp))
}

#[utoipa::path(
    get,
    path = "/v1/tts/voices/{id}",
    tag = "tts",
    params(("id" = Uuid, Path, description = "Voice ID")),
    responses(
        (status = 200, description = "Voice detail", body = VoiceResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Voice not found"),
    ),
    security(("bearer_auth" = []))
)]
async fn get_voice(
    State(state): State<TtsState>,
    _auth: JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<VoiceResponse>, HttpError> {
    let v = GetVoice {
        repository: state.repo.clone(),
    }
    .execute(id)
    .await
    .map_err(HttpError::from)?;

    Ok(Json(VoiceResponse {
        id: v.id,
        name: v.name,
        locale: v.locale,
        tier: v.tier.as_str().to_owned(),
        descriptor: v.descriptor,
        avatar_hue: v.avatar_hue,
        preview_url: v.preview_url,
        created_at: v.created_at,
    }))
}

#[utoipa::path(
    get,
    path = "/v1/tts/books/{file_id}/chunks",
    tag = "tts",
    params(
        ("file_id" = Uuid, Path, description = "Book file ID"),
        GetChunksQuery,
    ),
    responses(
        (status = 200, description = "TTS-ready text chunks", body = Vec<TextChunkResponse>),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
async fn get_chunks(
    State(state): State<TtsState>,
    _auth: JwtAuthExtractor,
    Path(file_id): Path<Uuid>,
    Query(q): Query<GetChunksQuery>,
) -> Result<Json<Vec<TextChunkResponse>>, HttpError> {
    let chunks = GetTextChunks {
        repository: state.repo.clone(),
    }
    .execute(file_id, q.chapter)
    .await
    .map_err(HttpError::from)?;

    let resp = chunks
        .into_iter()
        .map(|c| TextChunkResponse {
            chapter: c.chapter,
            sequence: c.sequence,
            text: c.text,
            char_count: c.char_count,
        })
        .collect();

    Ok(Json(resp))
}
