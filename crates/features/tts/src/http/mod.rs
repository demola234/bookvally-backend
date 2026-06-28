pub mod dto;
pub mod routes;

use utoipa::OpenApi;

use crate::http::dto::{TextChunkResponse, VoiceResponse};

#[derive(OpenApi)]
#[openapi(
    paths(routes::list_voices, routes::get_voice, routes::get_chunks,),
    components(schemas(VoiceResponse, TextChunkResponse,))
)]
pub struct TtsApiDoc;
