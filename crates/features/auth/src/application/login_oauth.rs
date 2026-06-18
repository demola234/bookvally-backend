use std::sync::Arc;
use kernel::{AppError, EventEnvelope};
use auth_kit::{JwtConfig, encode_access, encode_refresh};
use messaging::{KafkaProducer, AUTH_EVENTS};
use sha2::{Sha256, Digest};
use chrono::Utc;
use uuid::Uuid;

use crate::application::ports::AuthRepository;
use crate::domain::{Session, TokenPair};
use crate::events::{USER_LOGGED_IN, USER_REGISTERED, UserLoggedIn, UserRegistered};

pub struct LoginOAuth<R> {
    pub repository:       R,
    pub jwt:              JwtConfig,
    pub refresh_ttl_secs: i64,
    pub kafka:            Arc<KafkaProducer>,
}

impl<R: AuthRepository> LoginOAuth<R> {
    pub async fn execute(
        &self,
        provider: String,
        provider_account_id: String,
        email: Option<String>,
        display_name: String,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<TokenPair, AppError> {
        let (user_id, handle, is_new) = match self.repository
            .find_user_by_oauth(&provider, &provider_account_id)
            .await
            .map_err(AppError::internal)?
        {
            Some((id, handle)) => (id, handle, false),
            None => {
                let handle = slugify_handle(&display_name);
                let id = self.repository
                    .upsert_oauth_user(&provider, &provider_account_id, email.as_deref(), &handle)
                    .await
                    .map_err(AppError::internal)?;
                (id, handle, true)
            }
        };

        let access_token  = encode_access(&self.jwt, user_id, handle.clone()).map_err(AppError::internal)?;
        let refresh_token = encode_refresh(&self.jwt, user_id, handle.clone()).map_err(AppError::internal)?;

        let hash       = hex::encode(Sha256::digest(refresh_token.as_bytes()));
        let expires_at = Utc::now() + chrono::Duration::seconds(self.refresh_ttl_secs);

        let session = Session {
            id: Uuid::new_v4(),
            user_id,
            device_id: None,
            refresh_token_hash: hash,
            ip_address: ip,
            user_agent,
            expires_at,
            handle: Some(handle.clone()),
            revoked_at: None,
        };

        self.repository.create_session(&session).await.map_err(AppError::internal)?;

        let user_uuid = *user_id.as_uuid();
        if is_new {
            let envelope = EventEnvelope::new(USER_REGISTERED, UserRegistered { user_id: user_uuid, handle });
            let _ = self.kafka.publish(AUTH_EVENTS, &user_uuid.to_string(), &envelope).await;
        } else {
            let envelope = EventEnvelope::new(USER_LOGGED_IN, UserLoggedIn { user_id: user_uuid });
            let _ = self.kafka.publish(AUTH_EVENTS, &user_uuid.to_string(), &envelope).await;
        }

        Ok(TokenPair { access_token, refresh_token, expires_in: self.refresh_ttl_secs })
    }
}

fn slugify_handle(display_name: &str) -> String {
    let base: String = display_name
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    let suffix = &Uuid::new_v4().to_string()[..6];
    let base = base.trim_matches('_');
    let max_base = 20 - 1 - suffix.len();
    let base = &base[..base.len().min(max_base)];
    format!("{}_{}", base, suffix)
}
