use kernel::UserId;
use redis::aio::ConnectionManager;
use redis::cmd;

pub struct Presence {
    pub client: ConnectionManager,
}

impl Presence {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { client: conn }
    }

    pub async fn mark_online(
        conn: &mut ConnectionManager,
        user_id: UserId,
        ttl_secs: usize,
    ) -> anyhow::Result<()> {
        cmd("SET")
            .arg(format!("presence:{}", user_id))
            .arg("online")
            .arg("EX")
            .arg(ttl_secs)
            .query_async::<()>(conn)
            .await?;
        Ok(())
    }

    pub async fn mark_offline(
        conn: &mut ConnectionManager,
        user_id: UserId,
    ) -> anyhow::Result<bool> {
        let deleted_count: i64 = cmd("DEL")
            .arg(format!("presence:{}", user_id))
            .query_async(conn)
            .await?;
        Ok(deleted_count > 0)
    }

    pub async fn is_online(conn: &mut ConnectionManager, user_id: UserId) -> anyhow::Result<bool> {
        let status: Option<String> = cmd("GET")
            .arg(format!("presence:{}", user_id))
            .query_async(conn)
            .await?;
        Ok(status.is_some())
    }
}
