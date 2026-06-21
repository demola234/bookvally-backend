use async_trait::async_trait;
use uuid::Uuid;
use kernel::UserId;
use crate::domain::book::Book;
use crate::domain::book_file::{BookFile, BookFormat};

#[async_trait]
pub trait CatalogRepository: Send + Sync {
    async fn create_book(&self, book: &Book) -> anyhow::Result<Uuid>;
    async fn find_book(&self, book_id: &Uuid) -> anyhow::Result<Option<Book>>;
    async fn list_books(&self, user_id: Uuid) -> anyhow::Result<Vec<Book>>;
    async fn update_book(&self, book: &Book) -> anyhow::Result<()>;
    async fn delete_book(&self, book_id: &Uuid) -> anyhow::Result<()>;
    async fn find_book_by_title(&self, title: &str) -> anyhow::Result<Option<Book>>;
    async fn find_books_by_author(&self, author: &str) -> anyhow::Result<Vec<Book>>;
    async fn find_books_by_genre(&self, genre: &str) -> anyhow::Result<Vec<Book>>;

    // book files
    async fn create_book_file(&self, file: &BookFile) -> anyhow::Result<Uuid>;
    async fn find_book_file(&self, file_id: Uuid, user_id: Uuid) -> anyhow::Result<Option<BookFile>>;
    async fn list_book_files(&self, user_id: Uuid) -> anyhow::Result<Vec<BookFile>>;
    async fn update_book_file(&self, file: &BookFile) -> anyhow::Result<()>;
    async fn delete_book_file(&self, file_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;
}

#[async_trait]
pub trait FileParser: Send + Sync {
    async fn parse(&self, storage_key: &str, format: BookFormat) -> anyhow::Result<ParsedBook>;
}

pub struct ParsedBook {
    pub title:       Option<String>,
    pub author:      Option<String>,
    pub page_count:  Option<i32>,
    pub language:    Option<String>,
    pub cover_bytes: Option<Vec<u8>>,
}
