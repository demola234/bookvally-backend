use axum::Json;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use kernel::{AuthUser, UserId};
use serde_json::json;
use std::future::Future;

use crate::jwt::{decode_access, JwtConfig};

pub struct JwtAuthExtractor(pub AuthUser);

type Rejection = (StatusCode, Json<serde_json::Value>);

fn reject() -> Rejection {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "unauthorized"})),
    )
}

impl<S> FromRequestParts<S> for JwtAuthExtractor
where
    S: Send + Sync + AsRef<JwtConfig>,
{
    type Rejection = Rejection;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        // Clone config before async block so no &S reference crosses await
        let config = state.as_ref().clone();

        // Extract the Bearer token synchronously from parts before async block
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|t| t.to_owned());

        async move {
            let token = auth_header.ok_or_else(reject)?;
            let claims = decode_access(&config, &token).map_err(|_| reject())?;
            let user_id: UserId = claims.sub.parse().map_err(|_| reject())?;
            let user = AuthUser::new(user_id, claims.handle);
            Ok(JwtAuthExtractor(user))
        }
    }
}
