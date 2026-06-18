use std::time::Duration;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde::{Serialize, de::DeserializeOwned};
use kernel::EventEnvelope;

use super::codec;

pub struct KafkaProducer {
    inner: FutureProducer,
}

impl KafkaProducer {
    pub fn new(broker: &str) -> anyhow::Result<Self> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .set("message.timeout.ms", "5000")
            .create::<FutureProducer>()?;
        Ok(Self { inner: producer })
    }

    pub async fn publish<T: Serialize + DeserializeOwned>(
        &self,
        topic: &str,
        key: &str,
        envelope: &EventEnvelope<T>,
    ) -> anyhow::Result<()> {
        let bytes = codec::encode(envelope)?;
        let record = FutureRecord::to(topic).key(key).payload(&bytes);
        self.inner
            .send(record, Duration::ZERO)
            .await
            .map_err(|(e, _)| anyhow::anyhow!("kafka send failed: {e}"))?;
        Ok(())
    }
}
