use crate::domain::{Device, Session};
use async_trait::async_trait;
use kernel::UserId;
use uuid::Uuid;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    // returns (user_id, handle) for existing oauth user
    async fn find_user_by_oauth(
        &self,
        provider: &str,
        provider_account_id: &str,
    ) -> anyhow::Result<Option<(UserId, String)>>;
    async fn upsert_oauth_user(
        &self,
        provider: &str,
        provider_account_id: &str,
        email: Option<&str>,
        handle: &str,
    ) -> anyhow::Result<UserId>;

    async fn create_session(&self, session: &Session) -> anyhow::Result<()>;
    async fn find_session_by_token_hash(&self, hash: &str) -> anyhow::Result<Option<Session>>;
    async fn revoke_session(&self, session_id: Uuid) -> anyhow::Result<()>;
    async fn revoke_all_user_sessions(&self, user_id: UserId) -> anyhow::Result<()>;

    async fn upsert_device(&self, device: &Device) -> anyhow::Result<Uuid>;
}
