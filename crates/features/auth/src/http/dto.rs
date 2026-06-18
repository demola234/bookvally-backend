use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct OAuthLoginRequest {
    pub provider:            String,
    pub provider_account_id: String,
    pub email:               Option<String>,
    pub display_name:        Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub access_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RevokeRequest {
    pub access_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterDeviceRequest {
    pub platform:    String,
    pub device_name: Option<String>,
    pub push_token:  Option<String>,
    pub app_version: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token:  String,
    pub refresh_token: String,
    pub expires_in:    i64,
}
