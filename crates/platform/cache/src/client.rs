use redis::{aio::ConnectionManager, Client};

pub async fn build_redis_client(url: &str) -> anyhow::Result<ConnectionManager> {
    let client = Client::open(url)?;
    let manager = ConnectionManager::new(client).await?;
    Ok(manager)
}