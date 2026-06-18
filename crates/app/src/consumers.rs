use std::sync::Arc;
use tracing::info;

use crate::container::Container;

/// Spawn all Kafka consumer tasks.
/// todo(ademola): Add consumers here as features are implemented.
pub fn spawn_consumers(_container: Arc<Container>) {
    info!("consumers ready (none registered yet)");
}
