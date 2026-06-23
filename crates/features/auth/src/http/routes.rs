use auth_kit::JwtAuthExtractor;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use http_kit::{v1, HttpError};

use crate::application::{
    login_oauth::LoginOAuth, refresh_token::RefreshToken, register_device::RegisterDevice,
    revoke_session::RevokeSession,
};
use crate::http::dto::{
    OAuthLoginRequest, RefreshRequest, RegisterDeviceRequest, RevokeRequest, TokenResponse,
};
use crate::wiring::AuthState;

pub fn routes() -> Router<AuthState> {
    v1(Router::new()
        .route("/auth/oauth", post(login_oauth))
        .route("/auth/refresh", post(refresh_token))
        .route("/auth/logout", post(revoke_session))
        .route("/auth/devices", post(register_device)))
}

#[utoipa::path(
    post,
    path = "/v1/auth/oauth",
    tag = "auth",
    request_body = OAuthLoginRequest,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 422, description = "Unprocessable entity"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn login_oauth(
    State(state): State<AuthState>,
    Json(body): Json<OAuthLoginRequest>,
) -> Result<Json<TokenResponse>, HttpError> {
    let display_name = body.display_name.unwrap_or_else(|| "user".to_string());

    let uc = LoginOAuth {
        repository: state.repo.clone(),
        jwt: state.jwt.clone(),
        refresh_ttl_secs: state.refresh_ttl_secs,
        kafka: state.kafka.clone(),
    };

    let pair = uc
        .execute(
            body.provider,
            body.provider_account_id,
            body.email,
            display_name,
            None,
            None,
        )
        .await
        .map_err(|e| {
            tracing::error!("login_oauth error: {e:#}");
            HttpError::from(e)
        })?;

    Ok(Json(TokenResponse {
        access_token: pair.access_token,
        refresh_token: pair.refresh_token,
        expires_in: pair.expires_in,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = TokenResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn refresh_token(
    State(state): State<AuthState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, HttpError> {
    let uc = RefreshToken {
        repository: state.repo.clone(),
        jwt: state.jwt.clone(),
        refresh_ttl_secs: state.refresh_ttl_secs,
    };

    let pair = uc.execute(body.access_token).await.map_err(|e| {
        tracing::error!("refresh_token error: {e:#}");
        HttpError::from(e)
    })?;

    Ok(Json(TokenResponse {
        access_token: pair.access_token,
        refresh_token: pair.refresh_token,
        expires_in: pair.expires_in,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/auth/logout",
    tag = "auth",
    request_body = RevokeRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Session revoked"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    )
)]
pub async fn revoke_session(
    State(state): State<AuthState>,
    JwtAuthExtractor(auth_user): JwtAuthExtractor,
    Json(body): Json<RevokeRequest>,
) -> Result<StatusCode, HttpError> {
    let uc = RevokeSession {
        repository: state.repo.clone(),
        redis: state.redis.clone(),
        access_ttl_secs: state.access_ttl_secs,
    };

    uc.execute(auth_user.id(), body.access_token)
        .await
        .map_err(|e| {
            tracing::error!("revoke_session error: {e:#}");
            HttpError::from(e)
        })?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/v1/auth/devices",
    tag = "auth",
    request_body = RegisterDeviceRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Device registered"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn register_device(
    State(state): State<AuthState>,
    JwtAuthExtractor(auth_user): JwtAuthExtractor,
    Json(body): Json<RegisterDeviceRequest>,
) -> Result<StatusCode, HttpError> {
    let uc = RegisterDevice {
        repository: state.repo.clone(),
    };

    uc.execute(
        auth_user.id(),
        body.platform,
        body.device_name,
        body.push_token,
        body.app_version,
    )
    .await
    .map_err(|e| {
        tracing::error!("register_device error: {e:#}");
        HttpError::from(e)
    })?;

    Ok(StatusCode::NO_CONTENT)
}
