use std::sync::Arc;
use tracing::info;

use crate::container::Container;

/// Spawn periodic background jobs (streak rollover, league reset, etc).
/// todo(ademola): Add jobs here as features are implemented.
pub fn spawn_jobs(_container: Arc<Container>) {
    info!("background jobs ready (none registered yet)");
}
