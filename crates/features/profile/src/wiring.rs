use auth_kit::JwtConfig;
use persistence::PgPool;
use crate::adapters::PgProfileRepository;

#[derive(Clone)]
pub struct ProfileState {
    pub repo: PgProfileRepository,
    pub jwt:  JwtConfig,
}

impl ProfileState {
    pub fn new(pool: PgPool, jwt: JwtConfig) -> Self {
        Self { repo: PgProfileRepository::new(pool), jwt }
    }
}

impl AsRef<JwtConfig> for ProfileState {
    fn as_ref(&self) -> &JwtConfig { &self.jwt }
}
