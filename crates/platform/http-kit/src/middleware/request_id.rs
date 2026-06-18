use axum::http::{HeaderName, Request};
use tower_http::request_id::{MakeRequestId, RequestId, SetRequestIdLayer};
use uuid::Uuid;

#[derive(Clone)]
pub struct UuidRequestId;

impl MakeRequestId for UuidRequestId {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string().parse().ok()?;
        Some(RequestId::new(id))
    }
}

pub fn request_id_layer() -> SetRequestIdLayer<UuidRequestId> {
    SetRequestIdLayer::new(
        HeaderName::from_static("x-request-id"),
        UuidRequestId,
    )
}
