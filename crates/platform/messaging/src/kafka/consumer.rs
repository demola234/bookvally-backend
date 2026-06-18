use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;
use rdkafka::message::OwnedMessage;
use serde::{Serialize, de::DeserializeOwned};
use kernel::EventEnvelope;

use super::codec;

pub struct KafkaConsumer {
    inner: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(broker: &str, group_id: &str, topics: &[&str]) -> anyhow::Result<Self> {
        let consumer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .set("group.id", group_id)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .create::<StreamConsumer>()?;
        consumer.subscribe(topics)?;
        Ok(Self { inner: consumer })
    }

    pub async fn next_message(&self) -> anyhow::Result<OwnedMessage> {
        let msg = self.inner.recv().await?;
        Ok(msg.detach())
    }

    pub fn decode<T: Serialize + DeserializeOwned>(msg: &OwnedMessage) -> anyhow::Result<EventEnvelope<T>> {
        let bytes = msg.payload().ok_or_else(|| anyhow::anyhow!("empty payload"))?;
        codec::decode(bytes)
    }
}
