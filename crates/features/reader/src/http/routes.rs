// use axum::{Json, Router, routing::post};
// 
// use super::dto::{ProgressResponse, SaveProgressRequest};
// 
// /// Register reader HTTP routes.
// pub fn routes() -> Router {
//     Router::new().route("/v1/reader/progress", post(save_progress))
// }
// 
// #[utoipa::path(
//     post,
//     path = "/v1/reader/progress",
//     tag = "reader",
//     request_body = SaveProgressRequest,
//     responses(
//         (status = 200, description = "Progress saved", body = ProgressResponse),
//         (status = 401, description = "Unauthorized"),
//     )
// )]
// async fn save_progress(
//     Json(_body): Json<SaveProgressRequest>,
// ) -> Json<ProgressResponse> {
//     todo!()
// }
