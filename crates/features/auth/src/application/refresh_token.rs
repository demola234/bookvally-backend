use auth_kit::{encode_access, encode_refresh, JwtConfig};
use chrono::Utc;
use kernel::AppError;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::application::ports::AuthRepository;
use crate::domain::{Session, TokenPair};

pub struct RefreshToken<R> {
    pub repository: R,
    pub jwt: JwtConfig,
    pub refresh_ttl_secs: i64,
}

impl<R: AuthRepository> RefreshToken<R> {
    pub async fn execute(&self, token: String) -> Result<TokenPair, AppError> {
        let hash = hex::encode(Sha256::digest(token.as_bytes()));

        let session = match self
            .repository
            .find_session_by_token_hash(&hash)
            .await
            .map_err(AppError::internal)?
        {
            Some(session) => session,
            None => return Err(AppError::Unauthorized),
        };

        if !session.is_valid() {
            return Err(AppError::Unauthorized);
        }

        self.repository
            .revoke_session(session.id)
            .await
            .map_err(AppError::internal)?;

        let handle = session.handle.clone().ok_or(AppError::Unauthorized)?;
        let access_token = encode_access(&self.jwt, session.user_id, handle.clone())
            .map_err(AppError::internal)?;
        let refresh_token =
            encode_refresh(&self.jwt, session.user_id, handle).map_err(AppError::internal)?;

        let new_hash = hex::encode(Sha256::digest(refresh_token.as_bytes()));
        let expires_at = Utc::now() + chrono::Duration::seconds(self.refresh_ttl_secs);

        let new_session = Session {
            id: Uuid::new_v4(),
            user_id: session.user_id,
            device_id: session.device_id,
            refresh_token_hash: new_hash,
            ip_address: session.ip_address,
            user_agent: session.user_agent,
            expires_at,
            handle: session.handle,
            revoked_at: None,
        };

        self.repository
            .create_session(&new_session)
            .await
            .map_err(AppError::internal)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: self.refresh_ttl_secs,
        })
    }
}
