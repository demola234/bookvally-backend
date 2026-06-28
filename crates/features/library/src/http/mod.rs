pub mod dto;
pub mod routes;

use dto::*;
use routes::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        add_to_shelf,
        list_shelf,
        get_queue,
        get_shelf_item,
        update_shelf_item,
        remove_from_shelf,
        log_session,
        list_sessions,
        create_highlight,
        list_highlights,
        update_highlight,
        delete_highlight,
        create_bookmark,
        list_bookmarks,
        delete_bookmark,
    ),
    components(schemas(
        AddToShelfRequest,
        UpdateItemRequest,
        LogSessionRequest,
        CreateHighlightRequest,
        UpdateHighlightRequest,
        CreateBookmarkRequest,
        LibraryItemResponse,
        QueueItemResponse,
        ReadingSessionResponse,
        HighlightResponse,
        BookmarkResponse,
        CreatedResponse,
    ))
)]
pub struct LibraryApiDoc;
