use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub shutdown_timeout_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
    pub idle_timeout_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    pub brokers: Vec<String>,
    pub consumer_group: String,
    pub session_timeout_ms: u32,
    pub auto_offset_reset: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub access_ttl_secs: u64,
    pub refresh_ttl_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreaksConfig {
    pub daily_goal_minutes: u32,
    pub max_equipped_freezes: u8,
    pub rollover_cron: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelemetryConfig {
    pub service_name: String,
    pub service_version: String,
    pub otlp_endpoint: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    pub bucket: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
}

// root config

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server:    ServerConfig,
    pub database:  DatabaseConfig,
    pub redis:     RedisConfig,
    pub kafka:     KafkaConfig,
    pub auth:      AuthConfig,
    pub streaks:   StreaksConfig,
    pub telemetry: TelemetryConfig,
    pub storage:   Option<StorageConfig>,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let cfg = Config::builder()
            .add_source(File::with_name("config/default").required(true))
            .add_source(File::with_name(&format!("config/{env}")).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("__")
                    .separator("__"),
            )
            .add_source(
                Environment::default()
                    .try_parsing(true)
                    .with_list_parse_key("kafka.brokers")
                    .list_separator(","),
            )
            .build()?;

        cfg.try_deserialize()
    }
}

impl ServerConfig {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
