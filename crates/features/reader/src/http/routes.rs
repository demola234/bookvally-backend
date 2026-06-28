use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing::delete, routing::post, Json, Router};
use uuid::Uuid;

use auth_kit::JwtAuthExtractor;
use http_kit::{v1, HttpError};
use kernel::AppError;

use crate::application::add_bookmark::{AddBookmark, AddBookmarkInput};
use crate::application::create_highlight::{CreateHighlight, CreateHighlightInput};
use crate::application::end_session::{EndSession, EndSessionInput};
use crate::application::list_highlights::ListHighlights;
use crate::application::ports::ReaderRepository;
use crate::application::save_progress::{SaveProgress, SaveProgressInput};
use crate::domain::SessionMode;
use crate::http::dto::{
    AddBookmarkRequest, CreateHighlightRequest, CreatedResponse, EndSessionRequest,
    HighlightResponse, SaveProgressRequest,
};
use crate::wiring::ReaderState;

pub fn routes() -> Router<ReaderState> {
    v1(Router::new()
        .route("/reader/items/{id}/progress", post(save_progress))
        .route("/reader/items/{id}/sessions", post(end_session))
        .route(
            "/reader/items/{id}/highlights",
            post(create_highlight).get(list_highlights),
        )
        .route(
            "/reader/items/{id}/highlights/{hid}",
            delete(delete_highlight),
        )
        .route("/reader/items/{id}/bookmarks", post(add_bookmark)))
}

#[utoipa::path(
    post,
    path = "/v1/reader/items/{id}/progress",
    tag = "reader",
    params(("id" = Uuid, Path, description = "Library item ID")),
    request_body = SaveProgressRequest,
    responses(
        (status = 204, description = "Progress saved"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Library item not found"),
    ),
    security(("bearer_auth" = []))
)]
async fn save_progress(
    State(state): State<ReaderState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Json(body): Json<SaveProgressRequest>,
) -> Result<StatusCode, HttpError> {
    SaveProgress {
        repository: state.repo.clone(),
    }
    .execute(
        id,
        *user.id().as_uuid(),
        SaveProgressInput {
            current_page: body.current_page,
            current_locator: body.current_locator,
            progress_pct: body.progress_pct,
        },
    )
    .await
    .map_err(HttpError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/v1/reader/items/{id}/sessions",
    tag = "reader",
    params(("id" = Uuid, Path, description = "Library item ID")),
    request_body = EndSessionRequest,
    responses(
        (status = 201, description = "Session logged", body = CreatedResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Library item not found"),
        (status = 422, description = "Invalid session data"),
    ),
    security(("bearer_auth" = []))
)]
async fn end_session(
    State(state): State<ReaderState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Json(body): Json<EndSessionRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), HttpError> {
    let mode = SessionMode::try_from(body.mode.as_str())
        .map_err(|e| HttpError::from(AppError::UnprocessableEntity(e.to_string())))?;

    let session_id = EndSession {
        repository: state.repo.clone(),
        kafka: Arc::clone(&state.kafka),
    }
    .execute(
        id,
        *user.id().as_uuid(),
        EndSessionInput {
            mode,
            voice_id: body.voice_id,
            started_at: body.started_at,
            ended_at: body.ended_at,
            pages_read: body.pages_read,
            minutes: body.minutes,
        },
    )
    .await
    .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(CreatedResponse::new(session_id))))
}

#[utoipa::path(
    post,
    path = "/v1/reader/items/{id}/highlights",
    tag = "reader",
    params(("id" = Uuid, Path, description = "Library item ID")),
    request_body = CreateHighlightRequest,
    responses(
        (status = 201, description = "Highlight created", body = CreatedResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Library item not found"),
    ),
    security(("bearer_auth" = []))
)]
async fn create_highlight(
    State(state): State<ReaderState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateHighlightRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), HttpError> {
    let highlight_id = CreateHighlight {
        repository: state.repo.clone(),
    }
    .execute(
        id,
        *user.id().as_uuid(),
        CreateHighlightInput {
            color: body.color,
            locator_start: body.locator_start,
            locator_end: body.locator_end,
            selected_text: body.selected_text,
        },
    )
    .await
    .map_err(HttpError::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreatedResponse::new(highlight_id)),
    ))
}

#[utoipa::path(
    get,
    path = "/v1/reader/items/{id}/highlights",
    tag = "reader",
    params(("id" = Uuid, Path, description = "Library item ID")),
    responses(
        (status = 200, description = "Highlights list", body = Vec<HighlightResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Library item not found"),
    ),
    security(("bearer_auth" = []))
)]
async fn list_highlights(
    State(state): State<ReaderState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<HighlightResponse>>, HttpError> {
    let highlights = ListHighlights {
        repository: state.repo.clone(),
    }
    .execute(id, *user.id().as_uuid())
    .await
    .map_err(HttpError::from)?;

    let resp = highlights
        .into_iter()
        .map(|h| HighlightResponse {
            id: h.id,
            library_item_id: h.library_item_id,
            color: h.color,
            locator_start: h.locator_start,
            locator_end: h.locator_end,
            selected_text: h.selected_text,
            note: h.note,
            created_at: h.created_at,
        })
        .collect();

    Ok(Json(resp))
}

#[utoipa::path(
    delete,
    path = "/v1/reader/items/{id}/highlights/{hid}",
    tag = "reader",
    params(
        ("id" = Uuid, Path, description = "Library item ID"),
        ("hid" = Uuid, Path, description = "Highlight ID"),
    ),
    responses(
        (status = 204, description = "Highlight deleted"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
async fn delete_highlight(
    State(state): State<ReaderState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path((_id, hid)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, HttpError> {
    state
        .repo
        .delete_highlight(hid, *user.id().as_uuid())
        .await
        .map_err(|e| HttpError::from(AppError::internal(e)))?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/v1/reader/items/{id}/bookmarks",
    tag = "reader",
    params(("id" = Uuid, Path, description = "Library item ID")),
    request_body = AddBookmarkRequest,
    responses(
        (status = 201, description = "Bookmark added", body = CreatedResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Library item not found"),
        (status = 422, description = "Validation error"),
    ),
    security(("bearer_auth" = []))
)]
async fn add_bookmark(
    State(state): State<ReaderState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Json(body): Json<AddBookmarkRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), HttpError> {
    let bookmark_id = AddBookmark {
        repository: state.repo.clone(),
    }
    .execute(
        id,
        *user.id().as_uuid(),
        AddBookmarkInput {
            locator: body.locator,
            page: body.page,
            label: body.label,
        },
    )
    .await
    .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(CreatedResponse::new(bookmark_id))))
}
