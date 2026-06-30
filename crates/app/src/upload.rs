use std::sync::Arc;

use axum::extract::{Multipart, State};
use axum::Json;
use image::imageops::FilterType;
use image::GenericImageView;
use serde::Serialize;
use uuid::Uuid;

use auth_kit::{JwtAuthExtractor, JwtConfig};
use http_kit::HttpError;
use kernel::AppError;
use storage::StorageService;

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct UploadState {
    pub storage: Arc<dyn StorageService>,
    pub jwt: JwtConfig,
}

impl AsRef<JwtConfig> for UploadState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

// ── Response ──────────────────────────────────────────────────────────────────

#[derive(Serialize, utoipa::ToSchema)]
pub struct UploadResponse {
    pub url: String,
    pub blurhash: String,
    pub width: u32,
    pub height: u32,
}

// ── Handler ───────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/v1/upload",
    tag = "upload",
    request_body(content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Image uploaded", body = UploadResponse),
        (status = 400, description = "No image field or invalid image"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn upload_image(
    State(state): State<UploadState>,
    _auth: JwtAuthExtractor,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, HttpError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| HttpError::from(AppError::UnprocessableEntity(e.to_string())))?
    {
        if field.name() != Some("image") {
            continue;
        }

        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_owned();

        let bytes = field
            .bytes()
            .await
            .map_err(|e| HttpError::from(AppError::UnprocessableEntity(e.to_string())))?;

        let img = image::load_from_memory(&bytes).map_err(|e| {
            HttpError::from(AppError::UnprocessableEntity(format!("invalid image: {e}")))
        })?;

        let (width, height) = img.dimensions();

        let blurhash = generate_blurhash(&bytes)
            .map_err(|e| HttpError::from(AppError::internal(anyhow::anyhow!("{e}"))))?;

        let key = format!(
            "uploads/{}.{}",
            Uuid::new_v4(),
            ext_from_content_type(&content_type)
        );

        let url = state
            .storage
            .upload(&key, bytes.to_vec(), &content_type)
            .await
            .map_err(|e| HttpError::from(AppError::internal(anyhow::anyhow!("{e}"))))?;

        return Ok(Json(UploadResponse {
            url,
            blurhash,
            width,
            height,
        }));
    }

    Err(HttpError::from(AppError::UnprocessableEntity(
        "missing 'image' field in multipart body".into(),
    )))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn generate_blurhash(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let img = image::load_from_memory(bytes)?;
    // Downscale before encoding — hash is visually identical at any size, but
    // encoding cost scales with pixel count so shrink to ~100px first.
    let small = img.resize(100, 100, FilterType::Triangle);
    let (w, h) = small.dimensions();
    use image::EncodableLayout;
    let hash = blurhash::encode(4, 3, w, h, small.to_rgba8().as_bytes())?;
    Ok(hash)
}

fn ext_from_content_type(ct: &str) -> &str {
    match ct {
        "image/jpeg" | "image/jpg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        _ => "bin",
    }
}
