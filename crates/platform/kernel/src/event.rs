use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "T: serde::Serialize", deserialize = "T: serde::de::DeserializeOwned"))]

pub struct EventEnvelope<T> {
    pub id: Uuid,
    pub event_type:  String,
    pub occurred_at: DateTime<Utc>,
    pub payload: T,
}

impl <T> EventEnvelope<T> {
    pub fn new(event_type: &'static str, payload: T) -> Self {
        Self::at(Utc::now(), event_type, payload)
    }

    pub fn at(occurred_at: DateTime<Utc>, event_type: &'static str, payload: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            occurred_at,
            event_type: event_type.to_string(),
            payload,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn payload_mut(&mut self) -> &mut T {
        &mut self.payload
    }
}

