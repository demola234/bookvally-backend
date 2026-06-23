use cache::{ConnectionManager, TokenBlacklist};
use kernel::{AppError, UserId};
use sha2::{Digest, Sha256};

use crate::application::ports::AuthRepository;

pub struct RevokeSession<R> {
    pub repository: R,
    pub redis: ConnectionManager,
    pub access_ttl_secs: u64,
}

impl<R: AuthRepository> RevokeSession<R> {
    pub async fn execute(&self, user_id: UserId, access_token: String) -> Result<(), AppError> {
        let refresh_hash = hex::encode(Sha256::digest(access_token.as_bytes()));

        let session = match self
            .repository
            .find_session_by_token_hash(&refresh_hash)
            .await
            .map_err(AppError::internal)?
        {
            Some(s) => s,
            None => return Err(AppError::Unauthorized),
        };

        if session.user_id.as_uuid() != user_id.as_uuid() {
            return Err(AppError::Forbidden);
        }

        self.repository
            .revoke_session(session.id)
            .await
            .map_err(AppError::internal)?;

        let access_hash = hex::encode(Sha256::digest(access_token.as_bytes()));
        TokenBlacklist::new(self.redis.clone())
            .revoke(&access_hash, self.access_ttl_secs)
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
