use axum::Router;

/// Nest routes under /v1. Call this in each feature's wiring.rs.
pub fn v1<S: Clone + Send + Sync + 'static>(router: Router<S>) -> Router<S> {
    Router::new().nest("/v1", router)
}
