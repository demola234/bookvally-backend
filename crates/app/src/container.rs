use std::sync::Arc;
use anyhow::Result;
use auth_kit::JwtConfig;
use cache::ConnectionManager;
use messaging::KafkaProducer;
use persistence::PgPool;

use crate::config::AppConfig;

/// Shared application state — cloned cheaply via Arc into every handler.
#[derive(Clone)]
pub struct Container {
    pub config:   AppConfig,
    pub db:       PgPool,
    pub redis:    ConnectionManager,
    pub kafka:    Arc<KafkaProducer>,
    pub jwt:      JwtConfig,
}

impl Container {
    pub async fn build(config: AppConfig) -> Result<Arc<Self>> {
        let db = persistence::build_pg_pool(
            &config.database.url,
            config.database.max_connections,
        )
        .await?;

        let redis = cache::build_redis_client(&config.redis.url).await?;

        let brokers = config.kafka.brokers.join(",");
        let kafka = Arc::new(KafkaProducer::new(&brokers)?);

        let jwt = JwtConfig::new(
            config.auth.jwt_secret.clone(),
            config.auth.access_ttl_secs as i64,
            config.auth.refresh_ttl_secs as i64,
        );

        Ok(Arc::new(Self { config, db, redis, kafka, jwt }))
    }
}

/// Allow JwtAuthExtractor to pull JwtConfig from Container state.
impl AsRef<JwtConfig> for Container {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
