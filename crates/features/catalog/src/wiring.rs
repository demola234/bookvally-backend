use std::sync::Arc;
use auth_kit::JwtConfig;
use persistence::PgPool;
use storage::StorageService;

use crate::adapters::cloud_import::CloudImporter;
use crate::adapters::parser::BookFileParser;
use crate::adapters::repository::PgCatalogRepository;

#[derive(Clone)]
pub struct CatalogState {
    pub repo:     PgCatalogRepository,
    pub jwt:      JwtConfig,
    pub importer: Arc<CloudImporter>,
    pub parser:   Arc<BookFileParser>,
}

impl CatalogState {
    pub fn new(pool: PgPool, jwt: JwtConfig, storage: Arc<dyn StorageService>) -> Self {
        Self {
            repo:     PgCatalogRepository::new(pool),
            jwt,
            importer: Arc::new(CloudImporter::new(storage.clone())),
            parser:   Arc::new(BookFileParser { storage }),
        }
    }
}

impl AsRef<JwtConfig> for CatalogState {
    fn as_ref(&self) -> &JwtConfig { &self.jwt }
}
