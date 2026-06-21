use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::{get, post}};
use auth_kit::JwtAuthExtractor;
use cache::cache::{del, get_json, set_json};
use http_kit::{v1, HttpError};
use kernel::AppError;
use uuid::Uuid;

use crate::application::{
    get_book::GetBook,
    import_from_cloud::{ImportFromCloud, ImportFromCloudInput},
    parse_file::ParseFile,
    ports::CatalogRepository,
};
use crate::http::dto::*;
use crate::wiring::CatalogState;

const BOOKS_LIST_TTL: usize = 300;
const BOOK_TTL:       usize = 600;

fn books_list_key(user_id: Uuid) -> String { format!("catalog:books:{user_id}") }
fn book_key(id: Uuid) -> String            { format!("catalog:book:{id}") }

pub fn routes() -> Router<CatalogState> {
    v1(Router::new()
        .route("/catalog/books/import",     post(import_book))
        .route("/catalog/books",            get(list_books))
        .route("/catalog/books/{id}",       get(get_book).delete(delete_book))
        .route("/catalog/books/{id}/cover", get(get_book_cover))
        .route("/catalog/books/{id}/parse", post(parse_book))
    )
}

#[utoipa::path(post, path = "/v1/catalog/books/import", tag = "catalog",
    request_body = ImportBookRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 202, body = ImportBookResponse),
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Unsupported file format"),
    )
)]
pub async fn import_book(
    State(mut state): State<CatalogState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Json(body): Json<ImportBookRequest>,
) -> Result<(StatusCode, Json<ImportBookResponse>), HttpError> {
    let user_id = *user.id().as_uuid();

    let file_id = ImportFromCloud {
        repository: state.repo,
        importer:   state.importer,
    }
    .execute(user_id, ImportFromCloudInput {
        source_url:     body.source_url,
        file_name:      body.file_name,
        cloud_provider: body.cloud_provider,
    })
    .await
    .map_err(HttpError::from)?;

    del(&mut state.redis, &books_list_key(user_id)).await.ok();

    Ok((StatusCode::ACCEPTED, Json(ImportBookResponse {
        file_id,
        status:  "pending".into(),
        message: "import queued — call /parse when ready".into(),
    })))
}

#[utoipa::path(get, path = "/v1/catalog/books", tag = "catalog",
    security(("bearer_auth" = [])),
    responses((status = 200, body = Vec<BookResponse>), (status = 401, description = "Unauthorized"))
)]
pub async fn list_books(
    State(mut state): State<CatalogState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
) -> Result<Json<Vec<BookResponse>>, HttpError> {
    let user_id = *user.id().as_uuid();
    let key = books_list_key(user_id);

    if let Ok(Some(cached)) = get_json::<Vec<BookResponse>>(&mut state.redis, &key).await {
        return Ok(Json(cached));
    }

    let books = state.repo
        .list_books(user_id)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    let response: Vec<BookResponse> = books.into_iter().map(BookResponse::from).collect();
    set_json(&mut state.redis, &key, &response, BOOKS_LIST_TTL).await.ok();

    Ok(Json(response))
}

#[utoipa::path(get, path = "/v1/catalog/books/{id}", tag = "catalog",
    params(("id" = Uuid, Path, description = "Book ID")),
    security(("bearer_auth" = [])),
    responses((status = 200, body = BookResponse), (status = 404, description = "Not found"))
)]
pub async fn get_book(
    State(mut state): State<CatalogState>,
    JwtAuthExtractor(_user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<BookResponse>, HttpError> {
    let key = book_key(id);

    if let Ok(Some(cached)) = get_json::<BookResponse>(&mut state.redis, &key).await {
        return Ok(Json(cached));
    }

    let book = GetBook { repository: state.repo }
        .execute(&id)
        .await
        .map_err(HttpError::from)?;

    let response = BookResponse::from(book);
    set_json(&mut state.redis, &key, &response, BOOK_TTL).await.ok();

    Ok(Json(response))
}

#[utoipa::path(delete, path = "/v1/catalog/books/{id}", tag = "catalog",
    params(("id" = Uuid, Path, description = "Book ID")),
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Deleted"), (status = 404, description = "Not found"))
)]
pub async fn delete_book(
    State(mut state): State<CatalogState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, HttpError> {
    let user_id = *user.id().as_uuid();

    state.repo
        .delete_book(&id)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    del(&mut state.redis, &book_key(id)).await.ok();
    del(&mut state.redis, &books_list_key(user_id)).await.ok();

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/v1/catalog/books/{id}/cover", tag = "catalog",
    params(("id" = Uuid, Path, description = "Book ID")),
    security(("bearer_auth" = [])),
    responses((status = 200, body = String, description = "Cover URL"), (status = 404, description = "Not found"))
)]
pub async fn get_book_cover(
    State(mut state): State<CatalogState>,
    JwtAuthExtractor(_user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, HttpError> {
    let key = book_key(id);

    if let Ok(Some(cached)) = get_json::<BookResponse>(&mut state.redis, &key).await {
        return Ok(Json(serde_json::json!({ "cover_url": cached.cover_url })));
    }

    let book = GetBook { repository: state.repo }
        .execute(&id)
        .await
        .map_err(HttpError::from)?;

    let response = BookResponse::from(book);
    set_json(&mut state.redis, &key, &response, BOOK_TTL).await.ok();

    Ok(Json(serde_json::json!({ "cover_url": response.cover_url })))
}

#[utoipa::path(post, path = "/v1/catalog/books/{id}/parse", tag = "catalog",
    params(("id" = Uuid, Path, description = "BookFile ID")),
    security(("bearer_auth" = [])),
    responses((status = 202, description = "Parsing started"), (status = 404, description = "Not found"))
)]
pub async fn parse_book(
    State(mut state): State<CatalogState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, HttpError> {
    let user_id = *user.id().as_uuid();

    ParseFile {
        repository: state.repo,
        parser:     state.parser,
    }
    .execute(id, user_id)
    .await
    .map_err(HttpError::from)?;

    del(&mut state.redis, &book_key(id)).await.ok();
    del(&mut state.redis, &books_list_key(user_id)).await.ok();

    Ok(StatusCode::ACCEPTED)
}
