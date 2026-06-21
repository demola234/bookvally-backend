use uuid::Uuid;
use kernel::AppError;
use crate::application::ports::CatalogRepository;
use crate::domain::book::Book;

pub struct ListBooks<R> { pub repository: R }

impl<R: CatalogRepository> ListBooks<R> {
	pub async fn execute(&self, user_id: Uuid) -> Result<Vec<Book>, AppError> {
		self.repository
			.list_books(user_id)
			.await
			.map_err(AppError::internal)
	}
}

pub struct GetBook<R> { pub repository: R }

impl<R: CatalogRepository> GetBook<R> {
	pub async fn execute(&self, book_id: &Uuid) -> Result<Book, AppError> {
		self.repository.find_book(book_id)
			.await
			.map_err(AppError::internal)?
			.ok_or_else(|| AppError::not_found("book"))
	}
}