use kernel::EventEnvelope;
use serde::{Serialize, de::DeserializeOwned};

pub fn encode<T: Serialize + DeserializeOwned>(envelope: &EventEnvelope<T>) -> anyhow::Result<Vec<u8>> {
    Ok(serde_json::to_vec(envelope)?)
}

pub fn decode<T: Serialize + DeserializeOwned>(bytes: &[u8]) -> anyhow::Result<EventEnvelope<T>> {
    let value: serde_json::Value = serde_json::from_slice(bytes)?;
    Ok(serde_json::from_value(value)?)
}
