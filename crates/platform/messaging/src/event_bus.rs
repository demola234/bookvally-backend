use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use kernel::EventEnvelope;

use crate::kafka::producer::KafkaProducer;

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish<T>(&self, topic: &str, key: &str, envelope: &EventEnvelope<T>) -> anyhow::Result<()>
    where
        T: Serialize + DeserializeOwned + Send + Sync;
}

#[async_trait]
impl EventBus for KafkaProducer {
    async fn publish<T>(&self, topic: &str, key: &str, envelope: &EventEnvelope<T>) -> anyhow::Result<()>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
    {
        self.publish(topic, key, envelope).await
    }
}
