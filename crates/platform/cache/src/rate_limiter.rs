use redis::aio::ConnectionManager;
use redis::cmd;

pub struct RateLimiter {
    pub client: ConnectionManager,
}

impl RateLimiter {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { client: conn }
    }

    pub async fn check_rate_limit(
        &self,
        key: &str,
        limit: usize,
        expire: usize,
    ) -> anyhow::Result<bool> {
        let mut conn = self.client.clone();
        let count: usize = cmd("INCR").arg(key).query_async(&mut conn).await?;
        if count == 1 {
            cmd("EXPIRE").arg(key).arg(expire).query_async::<()>(&mut conn).await?;
        }
        Ok(count <= limit)
    }

    pub async fn reset_rate_limit(&self, key: &str) -> anyhow::Result<()> {
        let mut conn = self.client.clone();
        cmd("DEL").arg(key).query_async::<()>(&mut conn).await?;
        Ok(())
    }

}