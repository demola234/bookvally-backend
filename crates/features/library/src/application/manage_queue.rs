use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::{LibraryRepository, Page, Pagination};
use crate::domain::{LibraryStatus, QueueItem};

pub struct GetQueue<R> {
    pub repository: R,
}

impl<R: LibraryRepository> GetQueue<R> {
    pub async fn execute(
        &self,
        user_id: Uuid,
        pagination: Pagination,
    ) -> Result<Page<QueueItem>, AppError> {
        let page = self
            .repository
            .list_items(user_id, Some(LibraryStatus::Queued), &pagination)
            .await
            .map_err(AppError::internal)?;

        Ok(Page {
            items: page.items.iter().map(QueueItem::from).collect(),
            total: page.total,
            page: page.page,
            limit: page.limit,
            pages: page.pages,
        })
    }
}
