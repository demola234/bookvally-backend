use axum::http::Request;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub fn trace_layer() -> TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
    impl tower_http::trace::MakeSpan<axum::body::Body> + Clone,
    DefaultOnRequest,
    DefaultOnResponse,
> {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<axum::body::Body>| {
            let request_id = request
                .headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown");

            tracing::span!(
                Level::INFO,
                "http_request",
                method     = %request.method(),
                uri        = %request.uri(),
                request_id = request_id,
            )
        })
}
