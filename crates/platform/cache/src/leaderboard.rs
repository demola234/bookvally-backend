use redis::aio::ConnectionManager;
use redis::cmd;
use kernel::UserId;

pub struct Leaderboard {
    pub client: ConnectionManager,
}

impl Leaderboard {
    pub fn new(client: ConnectionManager) -> Self {
        Self { client }
    }

    pub async fn upsert_score(&self, league_id: &str, user_id: UserId, score: f64) -> anyhow::Result<()> {
        let mut conn = self.client.clone();
        cmd("ZADD").arg(format!("leaderboard:{}", league_id)).arg(score).arg(user_id.to_string()).query_async::<()>(&mut conn).await?;
        Ok(())
    }

    pub async fn top_n(&self, league_id: &str, n: usize) -> anyhow::Result<Vec<(String, f64)>> {
        let mut conn = self.client.clone();
        let result = cmd("ZREVRANGE")
            .arg(format!("leaderboard:{}", league_id))
            .arg(0)
            .arg(n.saturating_sub(1) as isize)
            .arg("WITHSCORES")
            .query_async(&mut conn)
            .await?;
        Ok(result)
    }

    pub async fn rank_of(&self, league_id: &str, user_id: &str) -> anyhow::Result<Option<usize>> {
        let mut conn = self.client.clone();
        let rank = cmd("ZREVRANK")
            .arg(format!("leaderboard:{}", league_id))
            .arg(user_id)
            .query_async(&mut conn)
            .await?;
        Ok(rank)
    }

    pub async fn score_of(&self, league_id: &str, user_id: &str) -> anyhow::Result<Option<f64>> {
        let mut conn = self.client.clone();
        let score = cmd("ZSCORE")
            .arg(format!("leaderboard:{}", league_id))
            .arg(user_id)
            .query_async(&mut conn)
            .await?;
        Ok(score)
    }

    pub async fn get_leaderboard(&self, league_id: &str) -> anyhow::Result<Vec<(String, f64)>> {
        let mut conn = self.client.clone();
        let result = cmd("ZREVRANGE")
            .arg(format!("leaderboard:{}", league_id))
            .arg(0)
            .arg(-1_isize)
            .arg("WITHSCORES")
            .query_async(&mut conn)
            .await?;
        Ok(result)
    }
}
