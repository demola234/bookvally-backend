use auth_kit::JwtConfig;
use cache::ConnectionManager;
use persistence::PgPool;
use std::sync::Arc;
use storage::StorageService;

use crate::adapters::cloud_import::CloudImporter;
use crate::adapters::parser::BookFileParser;
use crate::adapters::repository::PgCatalogRepository;

#[derive(Clone)]
pub struct CatalogState {
    pub repo: PgCatalogRepository,
    pub jwt: JwtConfig,
    pub redis: ConnectionManager,
    pub importer: Arc<CloudImporter>,
    pub parser: Arc<BookFileParser>,
}

impl CatalogState {
    pub fn new(
        pool: PgPool,
        jwt: JwtConfig,
        storage: Arc<dyn StorageService>,
        redis: ConnectionManager,
    ) -> Self {
        Self {
            repo: PgCatalogRepository::new(pool),
            jwt,
            importer: Arc::new(CloudImporter::new(storage.clone())),
            parser: Arc::new(BookFileParser { storage }),
            redis,
        }
    }
}

impl AsRef<JwtConfig> for CatalogState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
