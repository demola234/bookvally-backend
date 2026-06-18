use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{DailyActivity, Streak};

#[async_trait]
pub trait StreakRepository: Send + Sync {
    async fn get_or_create(&self, user_id: Uuid) -> anyhow::Result<Streak>;
    async fn save_streak(&self, streak: &Streak) -> anyhow::Result<()>;
    async fn get_or_create_activity(
        &self,
        user_id: Uuid,
        date: NaiveDate,
    ) -> anyhow::Result<DailyActivity>;
    async fn save_activity(&self, activity: &DailyActivity) -> anyhow::Result<()>;
    async fn count_available_freezes(&self, user_id: Uuid) -> anyhow::Result<i64>;
    async fn consume_one_freeze(&self, user_id: Uuid, used_on: NaiveDate) -> anyhow::Result<()>;
}
