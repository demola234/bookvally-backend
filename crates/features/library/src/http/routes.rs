use auth_kit::JwtAuthExtractor;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use http_kit::{v1, HttpError};
use kernel::AppError;
use uuid::Uuid;

use crate::application::{
    add_to_shelf::{AddToShelf, AddToShelfInput},
    log_session::{LogSession, LogSessionInput},
    manage_bookmarks::{CreateBookmark, CreateBookmarkInput, DeleteBookmark, ListBookmarks},
    manage_highlights::{
        CreateHighlight, CreateHighlightInput, DeleteHighlight, ListHighlights, UpdateHighlight,
        UpdateHighlightInput,
    },
    manage_queue::GetQueue,
    ports::{LibraryRepository, Pagination},
    update_items::{UpdateItem, UpdateItemInput},
};
use crate::domain::{AddedVia, LibraryStatus, SessionMode};
use crate::http::dto::*;
use crate::wiring::LibraryState;

pub fn routes() -> Router<LibraryState> {
    v1(Router::new()
        .route("/library", post(add_to_shelf).get(list_shelf))
        .route("/library/queue", get(get_queue))
        .route(
            "/library/{id}",
            get(get_shelf_item)
                .patch(update_shelf_item)
                .delete(remove_from_shelf),
        )
        .route(
            "/library/{id}/sessions",
            get(list_sessions).post(log_session),
        )
        .route(
            "/library/{id}/highlights",
            get(list_highlights).post(create_highlight),
        )
        .route(
            "/library/{id}/highlights/{hid}",
            patch(update_highlight).delete(delete_highlight),
        )
        .route(
            "/library/{id}/bookmarks",
            get(list_bookmarks).post(create_bookmark),
        )
        .route("/library/{id}/bookmarks/{bid}", delete(delete_bookmark)))
}

#[utoipa::path(post, path = "/v1/library", tag = "library",
    request_body = AddToShelfRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, body = CreatedResponse),
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Book already in library"),
    )
)]
pub async fn add_to_shelf(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Json(body): Json<AddToShelfRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), HttpError> {
    let added_via = AddedVia::try_from(body.added_via)
        .map_err(|e| HttpError::from(AppError::UnprocessableEntity(e.to_string())))?;

    let id = AddToShelf {
        repository: state.repo,
    }
    .execute(
        *user.id().as_uuid(),
        AddToShelfInput {
            book_id: body.book_id,
            book_file_id: body.book_file_id,
            added_via,
        },
    )
    .await
    .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

#[utoipa::path(get, path = "/v1/library", tag = "library",
    params(
        ("status" = Option<String>, Query, description = "Filter by status: queued|reading|finished|dropped"),
        ("page" = Option<i64>, Query, description = "Page number (default 1)"),
        ("limit" = Option<i64>, Query, description = "Items per page (default 20, max 100)"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Paginated library items"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn list_shelf(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Query(query): Query<ListItemsQuery>,
) -> Result<Json<PageResponse<LibraryItemResponse>>, HttpError> {
    let status = query
        .parsed_status()
        .map_err(|e| HttpError::from(AppError::UnprocessableEntity(e.to_string())))?;

    let pagination = Pagination::new(query.page, query.limit);
    let user_id = *user.id().as_uuid();

    let page = state
        .repo
        .list_items(user_id, status, &pagination)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok(Json(PageResponse::from(page)))
}

#[utoipa::path(get, path = "/v1/library/queue", tag = "library",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Queued books"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn get_queue(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PageResponse<QueueItemResponse>>, HttpError> {
    let pagination = Pagination::new(query.page, query.limit);

    let page = GetQueue {
        repository: state.repo,
    }
    .execute(*user.id().as_uuid(), pagination)
    .await
    .map_err(HttpError::from)?;

    Ok(Json(PageResponse::from(page)))
}

#[utoipa::path(get, path = "/v1/library/{id}", tag = "library",
    params(("id" = Uuid, Path, description = "Library item ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, body = LibraryItemResponse),
        (status = 404, description = "Not found"),
    )
)]
pub async fn get_shelf_item(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<LibraryItemResponse>, HttpError> {
    let item = state
        .repo
        .find_item(id, *user.id().as_uuid())
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError::from(AppError::not_found("library item")))?;

    Ok(Json(LibraryItemResponse::from(item)))
}

#[utoipa::path(patch, path = "/v1/library/{id}", tag = "library",
    params(("id" = Uuid, Path, description = "Library item ID")),
    request_body = UpdateItemRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, body = LibraryItemResponse),
        (status = 404, description = "Not found"),
        (status = 422, description = "Invalid input"),
    )
)]
pub async fn update_shelf_item(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateItemRequest>,
) -> Result<Json<LibraryItemResponse>, HttpError> {
    let status = body
        .status
        .map(LibraryStatus::try_from)
        .transpose()
        .map_err(|e| HttpError::from(AppError::UnprocessableEntity(e.to_string())))?;

    let item = UpdateItem {
        repository: state.repo,
    }
    .execute(
        id,
        *user.id().as_uuid(),
        UpdateItemInput {
            status,
            current_page: body.current_page,
            current_locator: body.current_locator,
            progress_pct: body.progress_pct,
            rating: body.rating,
        },
    )
    .await
    .map_err(HttpError::from)?;

    Ok(Json(LibraryItemResponse::from(item)))
}

#[utoipa::path(delete, path = "/v1/library/{id}", tag = "library",
    params(("id" = Uuid, Path, description = "Library item ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn remove_from_shelf(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, HttpError> {
    state
        .repo
        .delete_item(id, *user.id().as_uuid())
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/v1/library/{id}/sessions", tag = "library",
    params(("id" = Uuid, Path, description = "Library item ID")),
    request_body = LogSessionRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, body = CreatedResponse),
        (status = 404, description = "Not found"),
        (status = 422, description = "Invalid session times"),
    )
)]
pub async fn log_session(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Json(body): Json<LogSessionRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), HttpError> {
    let mode = SessionMode::try_from(body.mode)
        .map_err(|e| HttpError::from(AppError::UnprocessableEntity(e.to_string())))?;

    let session_id = LogSession {
        repository: state.repo,
    }
    .execute(
        id,
        *user.id().as_uuid(),
        LogSessionInput {
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

    Ok((
        StatusCode::CREATED,
        Json(CreatedResponse { id: session_id }),
    ))
}

#[utoipa::path(get, path = "/v1/library/{id}/sessions", tag = "library",
    params(
        ("id" = Uuid, Path, description = "Library item ID"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Paginated sessions"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn list_sessions(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PageResponse<ReadingSessionResponse>>, HttpError> {
    let pagination = Pagination::new(query.page, query.limit);
    let user_id = *user.id().as_uuid();

    let page = state
        .repo
        .list_sessions(id, user_id, &pagination)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok(Json(PageResponse::from(page)))
}

#[utoipa::path(post, path = "/v1/library/{id}/highlights", tag = "library",
    params(("id" = Uuid, Path, description = "Library item ID")),
    request_body = CreateHighlightRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, body = CreatedResponse),
        (status = 404, description = "Not found"),
    )
)]
pub async fn create_highlight(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateHighlightRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), HttpError> {
    let highlight_id = CreateHighlight {
        repository: state.repo,
    }
    .execute(
        id,
        *user.id().as_uuid(),
        CreateHighlightInput {
            color: body.color,
            locator_start: body.locator_start,
            locator_end: body.locator_end,
            selected_text: body.selected_text,
            note: body.note,
        },
    )
    .await
    .map_err(HttpError::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreatedResponse { id: highlight_id }),
    ))
}

#[utoipa::path(get, path = "/v1/library/{id}/highlights", tag = "library",
    params(
        ("id" = Uuid, Path, description = "Library item ID"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Paginated highlights"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn list_highlights(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PageResponse<HighlightResponse>>, HttpError> {
    let pagination = Pagination::new(query.page, query.limit);

    let page = ListHighlights {
        repository: state.repo,
    }
    .execute(id, *user.id().as_uuid(), pagination)
    .await
    .map_err(HttpError::from)?;

    Ok(Json(PageResponse::from(page)))
}

#[utoipa::path(patch, path = "/v1/library/{id}/highlights/{hid}", tag = "library",
    params(
        ("id" = Uuid, Path, description = "Library item ID"),
        ("hid" = Uuid, Path, description = "Highlight ID"),
    ),
    request_body = UpdateHighlightRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Updated"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn update_highlight(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path((_id, hid)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdateHighlightRequest>,
) -> Result<StatusCode, HttpError> {
    let note = if body.clear_note {
        Some(None)
    } else {
        body.note.map(Some)
    };

    UpdateHighlight {
        repository: state.repo,
    }
    .execute(
        hid,
        *user.id().as_uuid(),
        UpdateHighlightInput {
            color: body.color,
            note,
        },
    )
    .await
    .map_err(HttpError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/v1/library/{id}/highlights/{hid}", tag = "library",
    params(
        ("id" = Uuid, Path, description = "Library item ID"),
        ("hid" = Uuid, Path, description = "Highlight ID"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn delete_highlight(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path((_id, hid)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, HttpError> {
    DeleteHighlight {
        repository: state.repo,
    }
    .execute(hid, *user.id().as_uuid())
    .await
    .map_err(HttpError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/v1/library/{id}/bookmarks", tag = "library",
    params(("id" = Uuid, Path, description = "Library item ID")),
    request_body = CreateBookmarkRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, body = CreatedResponse),
        (status = 404, description = "Not found"),
        (status = 422, description = "Invalid input"),
    )
)]
pub async fn create_bookmark(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateBookmarkRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), HttpError> {
    let bookmark_id = CreateBookmark {
        repository: state.repo,
    }
    .execute(
        id,
        *user.id().as_uuid(),
        CreateBookmarkInput {
            locator: body.locator,
            page: body.page,
            label: body.label,
        },
    )
    .await
    .map_err(HttpError::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreatedResponse { id: bookmark_id }),
    ))
}

#[utoipa::path(get, path = "/v1/library/{id}/bookmarks", tag = "library",
    params(
        ("id" = Uuid, Path, description = "Library item ID"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Paginated bookmarks"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn list_bookmarks(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PageResponse<BookmarkResponse>>, HttpError> {
    let pagination = Pagination::new(query.page, query.limit);

    let page = ListBookmarks {
        repository: state.repo,
    }
    .execute(id, *user.id().as_uuid(), pagination)
    .await
    .map_err(HttpError::from)?;

    Ok(Json(PageResponse::from(page)))
}

#[utoipa::path(delete, path = "/v1/library/{id}/bookmarks/{bid}", tag = "library",
    params(
        ("id" = Uuid, Path, description = "Library item ID"),
        ("bid" = Uuid, Path, description = "Bookmark ID"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn delete_bookmark(
    State(state): State<LibraryState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path((id, bid)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, HttpError> {
    DeleteBookmark {
        repository: state.repo,
    }
    .execute(bid, id, *user.id().as_uuid())
    .await
    .map_err(HttpError::from)?;

    Ok(StatusCode::NO_CONTENT)
}
