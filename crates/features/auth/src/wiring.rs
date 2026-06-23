use crate::adapters::PgAuthRepository;
use auth_kit::JwtConfig;
use cache::ConnectionManager;
use messaging::KafkaProducer;
use persistence::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub repo: PgAuthRepository,
    pub jwt: JwtConfig,
    pub refresh_ttl_secs: i64,
    pub access_ttl_secs: u64,
    pub redis: ConnectionManager,
    pub kafka: Arc<KafkaProducer>,
}

impl AuthState {
    pub fn new(
        pool: PgPool,
        jwt: JwtConfig,
        refresh_ttl_secs: i64,
        access_ttl_secs: u64,
        redis: ConnectionManager,
        kafka: Arc<KafkaProducer>,
    ) -> Self {
        Self {
            repo: PgAuthRepository::new(pool),
            jwt,
            refresh_ttl_secs,
            access_ttl_secs,
            redis,
            kafka,
        }
    }
}

impl AsRef<JwtConfig> for AuthState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
