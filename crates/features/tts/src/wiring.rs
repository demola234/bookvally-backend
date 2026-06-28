use auth_kit::JwtConfig;
use persistence::PgPool;

use crate::adapters::PgTtsRepository;

#[derive(Clone)]
pub struct TtsState {
    pub repo: PgTtsRepository,
    pub jwt: JwtConfig,
}

impl TtsState {
    pub fn new(pool: PgPool, jwt: JwtConfig) -> Self {
        Self {
            repo: PgTtsRepository::new(pool),
            jwt,
        }
    }
}

impl AsRef<JwtConfig> for TtsState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
