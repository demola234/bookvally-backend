pub mod dto;
pub mod routes;

use dto::*;
use routes::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        import_book,
        list_books,
        get_book,
        delete_book,
        get_book_cover,
        parse_book
    ),
    components(schemas(
        ImportBookRequest,
        UpdateBookRequest,
        BookResponse,
        BookFileResponse,
        ImportBookResponse
    ))
)]
pub struct CatalogApiDoc;
