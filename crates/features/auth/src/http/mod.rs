pub mod dto;
pub mod routes;

use dto::*;
use routes::*;
use utoipa::OpenApi;

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
