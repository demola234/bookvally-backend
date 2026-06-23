use crate::application::ports::CatalogRepository;
use crate::domain::book::Book;
use kernel::AppError;
use uuid::Uuid;

pub struct AddBook<R> {
    pub repository: R,
}

impl<R: CatalogRepository> AddBook<R> {
    pub async fn execute(&self, book: &Book) -> Result<Uuid, AppError> {
        self.repository
            .create_book(book)
            .await
            .map_err(|e| AppError::internal(e.to_string()))
    }
}
