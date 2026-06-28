use auth_kit::JwtConfig;
use persistence::PgPool;

use crate::adapters::repository::PgLibraryRepository;

#[derive(Clone)]
pub struct LibraryState {
    pub repo: PgLibraryRepository,
    pub jwt: JwtConfig,
}

impl LibraryState {
    pub fn new(pool: PgPool, jwt: JwtConfig) -> Self {
        Self {
            repo: PgLibraryRepository::new(pool),
            jwt,
        }
    }
}

impl AsRef<JwtConfig> for LibraryState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
