use uuid::Uuid;

use kernel::AppError;

use crate::application::ports::LibraryRepository;
use crate::domain::shelf_entry::LibraryItem;
use crate::domain::AddedVia;

pub struct AddToShelfInput {
    pub book_id: Uuid,
    pub book_file_id: Option<Uuid>,
    pub added_via: AddedVia,
}

pub struct AddToShelf<R> {
    pub repository: R,
}

impl<R: LibraryRepository> AddToShelf<R> {
    pub async fn execute(&self, user_id: Uuid, input: AddToShelfInput) -> Result<Uuid, AppError> {
        let existing = self
            .repository
            .find_item_by_book(input.book_id, user_id)
            .await
            .map_err(AppError::internal)?;

        if existing.is_some() {
            return Err(AppError::UnprocessableEntity(
                "book already in library".into(),
            ));
        }

        let item = LibraryItem::new(user_id, input.book_id, input.book_file_id, input.added_via);

        self.repository
            .add_item(&item)
            .await
            .map_err(AppError::internal)
    }
}
