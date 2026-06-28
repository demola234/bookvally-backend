use std::sync::Arc;

use auth_kit::JwtConfig;
use messaging::KafkaProducer;
use persistence::PgPool;

use crate::adapters::repository::PgReaderRepository;

#[derive(Clone)]
pub struct ReaderState {
    pub repo: PgReaderRepository,
    pub jwt: JwtConfig,
    pub kafka: Arc<KafkaProducer>,
}

impl ReaderState {
    pub fn new(pool: PgPool, jwt: JwtConfig, kafka: Arc<KafkaProducer>) -> Self {
        Self {
            repo: PgReaderRepository::new(pool),
            jwt,
            kafka,
        }
    }
}

impl AsRef<JwtConfig> for ReaderState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
