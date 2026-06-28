pub mod dto;
pub mod routes;

use utoipa::OpenApi;

use crate::http::dto::{
    AddBookmarkRequest, BookmarkResponse, CreatedResponse, CreateHighlightRequest,
    EndSessionRequest, HighlightResponse, SaveProgressRequest,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::save_progress,
        routes::end_session,
        routes::create_highlight,
        routes::list_highlights,
        routes::delete_highlight,
        routes::add_bookmark,
    ),
    components(schemas(
        SaveProgressRequest,
        EndSessionRequest,
        CreateHighlightRequest,
        AddBookmarkRequest,
        CreatedResponse,
        HighlightResponse,
        BookmarkResponse,
    ))
)]
pub struct ReaderApiDoc;
