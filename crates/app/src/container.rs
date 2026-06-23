use anyhow::Result;
use auth_kit::JwtConfig;
use cache::ConnectionManager;
use messaging::KafkaProducer;
use persistence::PgPool;
use std::sync::Arc;
use storage::{CloudR2Storage, CloudR2StorageConfig, StorageService};

use crate::config::AppConfig;

#[derive(Clone)]
pub struct Container {
    pub config: AppConfig,
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub kafka: Arc<KafkaProducer>,
    pub jwt: JwtConfig,
    pub storage: Option<Arc<dyn StorageService>>,
}

impl Container {
    pub async fn build(config: AppConfig) -> Result<Arc<Self>> {
        let db = persistence::build_pg_pool(&config.database.url, config.database.max_connections)
            .await?;

        let redis = cache::build_redis_client(&config.redis.url).await?;

        let brokers = config.kafka.brokers.join(",");
        let kafka = Arc::new(KafkaProducer::new(&brokers)?);

        let jwt = JwtConfig::new(
            config.auth.jwt_secret.clone(),
            config.auth.access_ttl_secs as i64,
            config.auth.refresh_ttl_secs as i64,
        );

        let storage: Option<Arc<dyn StorageService>> = match &config.storage {
            Some(s) => {
                let r2 = CloudR2Storage::new(&CloudR2StorageConfig {
                    endpoint: s.endpoint.clone(),
                    bucket: s.bucket.clone(),
                    region: s.region.clone(),
                    access_key_id: s.access_key_id.clone(),
                    secret_access_key: s.secret_access_key.clone(),
                    public_url: s.public_url.clone(),
                })
                .await?;
                Some(Arc::new(r2))
            }
            None => {
                tracing::warn!("no storage config found");
                None
            }
        };

        Ok(Arc::new(Self {
            config,
            db,
            redis,
            kafka,
            jwt,
            storage,
        }))
    }
}

impl AsRef<JwtConfig> for Container {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
