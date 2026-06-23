use kernel::UserId;
use redis::aio::ConnectionManager;

pub struct SessionStore {
    pub client: ConnectionManager,
}

impl SessionStore {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { client: conn }
    }

    pub async fn store(
        &self,
        token: &str,
        user_id: UserId,
        ttl: usize,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.client.clone();
        redis::cmd("SET")
            .arg(format!("session:{}", token))
            .arg(user_id.to_string())
            .arg("EX")
            .arg(ttl)
            .query_async::<()>(&mut conn)
            .await?;
        Ok(())
    }

    pub async fn get(&self, token: &str) -> Result<Option<UserId>, redis::RedisError> {
        let mut conn = self.client.clone();
        let user_id: Option<String> = redis::cmd("GET")
            .arg(format!("session:{}", token))
            .query_async(&mut conn)
            .await?;
        Ok(user_id.map(|id| id.parse().unwrap()))
    }

    pub async fn destroy(&self, token: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.client.clone();
        redis::cmd("DEL")
            .arg(format!("session:{}", token))
            .query_async::<()>(&mut conn)
            .await?;
        Ok(())
    }
}
