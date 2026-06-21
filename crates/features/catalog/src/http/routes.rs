use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, post}};
use auth_kit::JwtAuthExtractor;
use http_kit::{v1, HttpError};
use kernel::AppError;
use uuid::Uuid;

use crate::application::{
    add_book::AddBook,
    get_book::GetBook,
    import_from_cloud::{ImportFromCloud, ImportFromCloudInput},
    parse_file::ParseFile,
    ports::CatalogRepository,
};
use crate::http::dto::*;
use crate::wiring::CatalogState;

pub fn routes() -> Router<CatalogState> {
    v1(Router::new()
        .route("/catalog/books/import",      post(import_book))
        .route("/catalog/books",             get(list_books))
        .route("/catalog/books/{id}",        get(get_book).delete(delete_book))
        .route("/catalog/books/{id}/cover",  get(get_book_cover))
        .route("/catalog/books/{id}/parse",  post(parse_book))
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
    State(state): State<CatalogState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Json(body): Json<ImportBookRequest>,
) -> Result<(StatusCode, Json<ImportBookResponse>), HttpError> {
    let uc = ImportFromCloud {
        repository: state.repo,
        importer:   state.importer,
    };

    let file_id = uc.execute(*user.id().as_uuid(), ImportFromCloudInput {
        source_url:     body.source_url,
        file_name:      body.file_name,
        cloud_provider: body.cloud_provider,
    })
    .await
    .map_err(HttpError::from)?;

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
    State(state): State<CatalogState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
) -> Result<Json<Vec<BookResponse>>, HttpError> {
    let books = state.repo
        .list_books(*user.id().as_uuid())
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok(Json(books.into_iter().map(BookResponse::from).collect()))
}

#[utoipa::path(get, path = "/v1/catalog/books/{id}", tag = "catalog",
    params(("id" = Uuid, Path, description = "Book ID")),
    security(("bearer_auth" = [])),
    responses((status = 200, body = BookResponse), (status = 404, description = "Not found"))
)]
pub async fn get_book(
    State(state): State<CatalogState>,
    JwtAuthExtractor(_user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<BookResponse>, HttpError> {
    let book = GetBook { repository: state.repo }
        .execute(&id)
        .await
        .map_err(HttpError::from)?;

    Ok(Json(BookResponse::from(book)))
}

#[utoipa::path(delete, path = "/v1/catalog/books/{id}", tag = "catalog",
    params(("id" = Uuid, Path, description = "Book ID")),
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Deleted"), (status = 404, description = "Not found"))
)]
pub async fn delete_book(
    State(state): State<CatalogState>,
    JwtAuthExtractor(_user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, HttpError> {
    state.repo
        .delete_book(&id)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/v1/catalog/books/{id}/cover", tag = "catalog",
    params(("id" = Uuid, Path, description = "Book ID")),
    security(("bearer_auth" = [])),
    responses((status = 200, body = String, description = "Cover URL"), (status = 404, description = "Not found"))
)]
pub async fn get_book_cover(
    State(state): State<CatalogState>,
    JwtAuthExtractor(_user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, HttpError> {
    let book = GetBook { repository: state.repo }
        .execute(&id)
        .await
        .map_err(HttpError::from)?;

    Ok(Json(serde_json::json!({ "cover_url": book.cover_url })))
}

#[utoipa::path(post, path = "/v1/catalog/books/{id}/parse", tag = "catalog",
    params(("id" = Uuid, Path, description = "BookFile ID")),
    security(("bearer_auth" = [])),
    responses((status = 202, description = "Parsing started"), (status = 404, description = "Not found"))
)]
pub async fn parse_book(
    State(state): State<CatalogState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, HttpError> {
    ParseFile {
        repository: state.repo,
        parser:     state.parser,
    }
    .execute(id, *user.id().as_uuid())
    .await
    .map_err(HttpError::from)?;

    Ok(StatusCode::ACCEPTED)
}
