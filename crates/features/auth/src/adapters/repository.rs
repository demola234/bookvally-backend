use crate::application::ports::AuthRepository;
use crate::domain::{Device, Session};
use async_trait::async_trait;
use kernel::UserId;
use persistence::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgAuthRepository {
    pool: PgPool,
}

impl PgAuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for PgAuthRepository {
    async fn find_user_by_oauth(
        &self,
        provider: &str,
        provider_account_id: &str,
    ) -> anyhow::Result<Option<(UserId, String)>> {
        let row = sqlx::query_as::<_, (Uuid, String)>(
            r#"SELECT u.id, u.handle
               FROM oauth_accounts oa
               JOIN users u ON u.id = oa.user_id
               WHERE oa.provider = $1::oauth_provider AND oa.provider_account_id = $2"#,
        )
        .bind(provider)
        .bind(provider_account_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(id, handle)| (UserId::from(id), handle)))
    }

    async fn upsert_oauth_user(
        &self,
        provider: &str,
        provider_account_id: &str,
        email: Option<&str>,
        handle: &str,
    ) -> anyhow::Result<UserId> {
        let user_id = sqlx::query_scalar::<_, Uuid>(
            r#"WITH ins_user AS (
                   INSERT INTO users (handle)
                   VALUES ($1)
                   ON CONFLICT (handle) DO UPDATE SET handle = EXCLUDED.handle
                   RETURNING id
               ), ins_oauth AS (
                   INSERT INTO oauth_accounts (user_id, provider, provider_account_id, email)
                   SELECT id, $2::oauth_provider, $3, $4 FROM ins_user
                   ON CONFLICT (provider, provider_account_id)
                   DO UPDATE SET email = EXCLUDED.email
                   RETURNING user_id
               )
               SELECT user_id FROM ins_oauth"#,
        )
        .bind(handle)
        .bind(provider)
        .bind(provider_account_id)
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(UserId::from(user_id))
    }

    async fn create_session(&self, session: &Session) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO sessions (id, user_id, device_id, refresh_token_hash, ip_address, user_agent, expires_at, handle)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        )
            .bind(session.id)
            .bind(*session.user_id.as_uuid())
            .bind(session.device_id)
            .bind(&session.refresh_token_hash)
            .bind(&session.ip_address)
            .bind(&session.user_agent)
            .bind(session.expires_at)
            .bind(&session.handle)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_session_by_token_hash(&self, hash: &str) -> anyhow::Result<Option<Session>> {
        let row = sqlx::query_as::<_, Session>(
            r#"SELECT * FROM sessions WHERE refresh_token_hash = $1 AND revoked_at IS NULL"#,
        )
        .bind(hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn revoke_session(&self, session_id: Uuid) -> anyhow::Result<()> {
        sqlx::query(r#"UPDATE sessions SET revoked_at = now() WHERE id = $1"#)
            .bind(session_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn revoke_all_user_sessions(&self, user_id: UserId) -> anyhow::Result<()> {
        sqlx::query(r#"UPDATE sessions SET revoked_at = now() WHERE user_id = $1"#)
            .bind(*user_id.as_uuid())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn upsert_device(&self, device: &Device) -> anyhow::Result<Uuid> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"INSERT INTO devices (id, user_id, device_name, platform, push_token, app_version, last_seen_at)
               VALUES ($1, $2, $3, $4::device_platform, $5, $6, COALESCE($7, now()))
               ON CONFLICT (id) DO UPDATE SET last_seen_at = now()
               RETURNING id"#,
        )
            .bind(device.id)
            .bind(*device.user_id.as_uuid())
            .bind(&device.device_name)
            .bind(&device.platform)
            .bind(&device.push_token)
            .bind(&device.app_version)
            .bind(device.last_seen_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(id)
    }
}
