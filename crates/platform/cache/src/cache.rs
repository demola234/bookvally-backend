use redis::{aio::ConnectionManager, cmd};
use serde::{de::DeserializeOwned, Serialize};

pub async fn set_json<T: Serialize>(
    conn: &mut ConnectionManager,
    key: &str,
    value: &T,
    ttl_secs: usize,
) -> anyhow::Result<()> {
    let json = serde_json::to_string(value)?;
    cmd("SET")
        .arg(key)
        .arg(json)
        .arg("EX")
        .arg(ttl_secs)
        .query_async::<()>(conn)
        .await?;
    Ok(())
}

pub async fn get_json<T: DeserializeOwned>(
    conn: &mut ConnectionManager,
    key: &str,
) -> anyhow::Result<Option<T>> {
    let raw: Option<String> = cmd("GET").arg(key).query_async(conn).await?;
    match raw {
        Some(s) => Ok(Some(serde_json::from_str::<T>(&s)?)),
        None => Ok(None),
    }
}

pub async fn del(conn: &mut ConnectionManager, key: &str) -> anyhow::Result<()> {
    cmd("DEL").arg(key).query_async::<()>(conn).await?;
    Ok(())
}
