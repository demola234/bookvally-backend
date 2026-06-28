use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::{LibraryRepository, Page, Pagination};
use crate::domain::Bookmark;

pub struct CreateBookmarkInput {
    pub locator: String,
    pub page: Option<i32>,
    pub label: Option<String>,
}

pub struct CreateBookmark<R> {
    pub repository: R,
}

impl<R: LibraryRepository> CreateBookmark<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        input: CreateBookmarkInput,
    ) -> Result<Uuid, AppError> {
        self.repository
            .find_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        let bookmark = Bookmark::new(item_id, input.locator, input.page, input.label)
            .map_err(|e| AppError::UnprocessableEntity(e.to_string()))?;

        self.repository
            .create_bookmark(&bookmark)
            .await
            .map_err(AppError::internal)
    }
}

pub struct ListBookmarks<R> {
    pub repository: R,
}

impl<R: LibraryRepository> ListBookmarks<R> {
    pub async fn execute(
        &self,
        item_id: Uuid,
        user_id: Uuid,
        pagination: Pagination,
    ) -> Result<Page<Bookmark>, AppError> {
        self.repository
            .find_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        self.repository
            .list_bookmarks(item_id, &pagination)
            .await
            .map_err(AppError::internal)
    }
}

pub struct DeleteBookmark<R> {
    pub repository: R,
}

impl<R: LibraryRepository> DeleteBookmark<R> {
    pub async fn execute(
        &self,
        bookmark_id: Uuid,
        item_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        self.repository
            .find_item(item_id, user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("library item"))?;

        self.repository
            .delete_bookmark(bookmark_id, item_id)
            .await
            .map_err(AppError::internal)
    }
}
