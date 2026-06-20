use std::sync::Arc;
use tracing::{error, info};

use messaging::{KafkaConsumer, AUTH_EVENTS};
use feat_auth::events::{USER_REGISTERED, UserRegistered};
use feat_profile::consumers::ProfileEventConsumer;
use feat_profile::adapters::PgProfileRepository;

use crate::container::Container;

pub fn spawn_consumers(container: Arc<Container>) {
    spawn_profile_consumer(container);
}

fn spawn_profile_consumer(container: Arc<Container>) {
    let pool   = container.db.clone();
    let broker = container.config.kafka.brokers.join(",");

    tokio::spawn(async move {
        let consumer = match KafkaConsumer::new(&broker, "profile-service", &[AUTH_EVENTS]) {
            Ok(c)  => c,
            Err(e) => { error!("profile consumer failed to start: {e:#}"); return; }
        };

        let handler = ProfileEventConsumer {
            repository: PgProfileRepository::new(pool),
        };

        info!("profile consumer started, listening on {AUTH_EVENTS}");

        loop {
            let msg = match consumer.next_message().await {
                Ok(m)  => m,
                Err(e) => { error!("kafka recv error: {e:#}"); continue; }
            };

            let envelope = match KafkaConsumer::decode::<serde_json::Value>(&msg) {
                Ok(e)  => e,
                Err(e) => { error!("decode error: {e:#}"); continue; }
            };

            if envelope.event_type != USER_REGISTERED {
                continue;
            }

            let payload: UserRegistered = match serde_json::from_value(
                serde_json::to_value(envelope.payload()).unwrap_or_default()
            ) {
                Ok(p)  => p,
                Err(e) => { error!("deserialize UserRegistered error: {e:#}"); continue; }
            };

            if let Err(e) = handler.handle_user_registered(payload.user_id, payload.handle).await {
                error!("handle_user_registered error: {e:#}");
            }
        }
    });
}
