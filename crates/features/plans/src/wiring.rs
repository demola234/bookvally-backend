use auth_kit::JwtConfig;
use persistence::PgPool;

use crate::adapters::repository::PgPlansRepository;

#[derive(Clone)]
pub struct PlansState {
    pub repo: PgPlansRepository,
    pub jwt: JwtConfig,
}

impl PlansState {
    pub fn new(db: PgPool, jwt: JwtConfig) -> Self {
        Self {
            repo: PgPlansRepository::new(db),
            jwt,
        }
    }
}

impl AsRef<JwtConfig> for PlansState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
