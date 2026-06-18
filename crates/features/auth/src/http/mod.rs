pub mod dto;
pub mod routes;

use utoipa::OpenApi;
use dto::*;
use routes::*;

#[derive(OpenApi)]
#[openapi(
    paths(login_oauth, refresh_token, revoke_session, register_device),
    components(schemas(
        OAuthLoginRequest,
        RefreshRequest,
        RevokeRequest,
        RegisterDeviceRequest,
        TokenResponse,
    ))
)]
pub struct AuthApiDoc;
