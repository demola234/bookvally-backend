use redis::{AsyncCommands, aio::ConnectionManager};

pub struct TokenBlacklist(pub ConnectionManager);

impl TokenBlacklist {
    pub fn new(conn: ConnectionManager) -> Self {
        Self(conn)
    }

    pub async fn revoke(&mut self, token_hash: &str, ttl_secs: u64) -> anyhow::Result<()> {
        let key = format!("blacklist:{token_hash}");
        self.0.set_ex::<_, _, ()>(key, 1u8, ttl_secs).await?;
        Ok(())
    }

    pub async fn is_revoked(&mut self, token_hash: &str) -> anyhow::Result<bool> {
        let key = format!("blacklist:{token_hash}");
        let exists: bool = self.0.exists(key).await?;
        Ok(exists)
    }
}
