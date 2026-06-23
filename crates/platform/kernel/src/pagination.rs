use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, total: i64, page: u32, per_page: u32, total_pages: u32) -> Self {
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.items
    }

    pub fn page(&self) -> u32 {
        self.page
    }

    pub fn total_pages(&self) -> u32 {
        self.total_pages
    }

    pub fn per_page(&self) -> u32 {
        self.per_page
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    #[serde(default = "PageRequest::default_page")]
    page: u32,
    #[serde(default = "PageRequest::default_per_page")]
    per_page: u32,
}

impl PageRequest {
    pub fn new(page: u32, per_page: u32) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.clamp(1, 100),
        }
    }

    pub fn offset(&self) -> i64 {
        (self.page.max(1) - 1) as i64 * self.limit()
    }

    pub fn limit(&self) -> i64 {
        self.per_page.clamp(1, 100) as i64
    }

    pub fn clamp(self) -> Self {
        Self {
            page: self.page.clamp(1, u32::MAX),
            per_page: self.per_page.clamp(1, 100),
        }
    }

    pub fn default_page() -> u32 {
        1
    }

    pub fn default_per_page() -> u32 {
        20
    }
}

impl Display for PageRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PageRequest {{ page: {}, per_page: {} }}",
            self.page, self.per_page
        )
    }
}
