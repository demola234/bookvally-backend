use std::sync::Arc;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::application::ports::StreakRepository;

pub struct RecordDailyActivity<R> {
    repo: Arc<R>,
    goal_minutes: Decimal,
}

impl<R: StreakRepository> RecordDailyActivity<R> {
    pub fn new(repo: Arc<R>, goal_minutes: Decimal) -> Self {
        Self { repo, goal_minutes }
    }

    /// Called after each reading session completes.
    /// Accumulates into the day's activity; advances or freezes the streak
    /// if the daily goal is newly met or missed.
    pub async fn execute(
        &self,
        user_id: Uuid,
        date: NaiveDate,
        minutes: Decimal,
        pages: i32,
        xp: i32,
    ) -> anyhow::Result<()> {
        let mut activity = self.repo.get_or_create_activity(user_id, date).await?;
        let newly_met = activity.accumulate(minutes, pages, xp, self.goal_minutes);
        self.repo.save_activity(&activity).await?;

        if newly_met {
            let mut streak = self.repo.get_or_create(user_id).await?;
            streak.advance(date)?;
            self.repo.save_streak(&streak).await?;
        }

        Ok(())
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;
    use std::sync::Mutex;

    use crate::domain::{DailyActivity, Streak};

    // ── minimal manual mock ───────────────────────────────────

    #[derive(Default)]
    struct MockRepo {
        streak: Mutex<Option<Streak>>,
        activities: Mutex<std::collections::HashMap<NaiveDate, DailyActivity>>,
        freeze_count: Mutex<i64>,
        freeze_used: Mutex<Option<NaiveDate>>,
        saves_streak: Mutex<u32>,
        saves_activity: Mutex<u32>,
    }

    #[async_trait]
    impl StreakRepository for MockRepo {
        async fn get_or_create(&self, user_id: Uuid) -> anyhow::Result<Streak> {
            Ok(self
                .streak
                .lock()
                .unwrap()
                .clone()
                .unwrap_or_else(|| Streak::new(user_id)))
        }

        async fn save_streak(&self, s: &Streak) -> anyhow::Result<()> {
            *self.streak.lock().unwrap() = Some(s.clone());
            *self.saves_streak.lock().unwrap() += 1;
            Ok(())
        }

        async fn get_or_create_activity(
            &self,
            user_id: Uuid,
            date: NaiveDate,
        ) -> anyhow::Result<DailyActivity> {
            Ok(self
                .activities
                .lock()
                .unwrap()
                .get(&date)
                .cloned()
                .unwrap_or_else(|| DailyActivity::new(user_id, date)))
        }

        async fn save_activity(&self, a: &DailyActivity) -> anyhow::Result<()> {
            self.activities
                .lock()
                .unwrap()
                .insert(a.activity_date, a.clone());
            *self.saves_activity.lock().unwrap() += 1;
            Ok(())
        }

        async fn count_available_freezes(&self, _: Uuid) -> anyhow::Result<i64> {
            Ok(*self.freeze_count.lock().unwrap())
        }

        async fn consume_one_freeze(&self, _: Uuid, date: NaiveDate) -> anyhow::Result<()> {
            *self.freeze_used.lock().unwrap() = Some(date);
            Ok(())
        }
    }

    fn date(d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 6, d).unwrap()
    }
    fn uid() -> Uuid {
        Uuid::new_v4()
    }
    fn goal() -> Decimal {
        dec!(15)
    }

    fn use_case(repo: Arc<MockRepo>) -> RecordDailyActivity<MockRepo> {
        RecordDailyActivity::new(repo, goal())
    }

    // ── goal not yet met ─────────────────────────────────────

    #[tokio::test]
    async fn below_goal_saves_activity_but_not_streak() {
        let repo = Arc::new(MockRepo::default());
        let uc = use_case(repo.clone());

        uc.execute(uid(), date(1), dec!(10), 5, 20).await.unwrap();

        assert_eq!(*repo.saves_activity.lock().unwrap(), 1);
        assert_eq!(
            *repo.saves_streak.lock().unwrap(),
            0,
            "streak must not be touched below goal"
        );
    }

    // ── goal newly met ───────────────────────────────────────

    #[tokio::test]
    async fn meeting_goal_advances_streak() {
        let repo = Arc::new(MockRepo::default());
        let uc = use_case(repo.clone());

        uc.execute(uid(), date(1), dec!(15), 10, 30).await.unwrap();

        assert_eq!(*repo.saves_streak.lock().unwrap(), 1);
        let streak = repo.streak.lock().unwrap().clone().unwrap();
        assert_eq!(streak.current_length, 1);
    }

    #[tokio::test]
    async fn meeting_goal_across_two_sessions_advances_streak_once() {
        let uid = uid();
        let repo = Arc::new(MockRepo::default());
        let uc = use_case(repo.clone());

        // First session: 10 min (not enough)
        uc.execute(uid, date(1), dec!(10), 5, 10).await.unwrap();
        assert_eq!(*repo.saves_streak.lock().unwrap(), 0);

        // Second session: 6 more min — crosses goal
        uc.execute(uid, date(1), dec!(6), 3, 12).await.unwrap();
        assert_eq!(*repo.saves_streak.lock().unwrap(), 1);

        // Third session same day: already met, no extra advance
        uc.execute(uid, date(1), dec!(5), 2, 10).await.unwrap();
        assert_eq!(
            *repo.saves_streak.lock().unwrap(),
            1,
            "streak advanced only once per day"
        );
    }

    #[tokio::test]
    async fn streak_advances_on_consecutive_days() {
        let uid = uid();
        let repo = Arc::new(MockRepo::default());
        let uc = use_case(repo.clone());

        uc.execute(uid, date(1), dec!(20), 0, 0).await.unwrap();
        uc.execute(uid, date(2), dec!(20), 0, 0).await.unwrap();
        uc.execute(uid, date(3), dec!(20), 0, 0).await.unwrap();

        let streak = repo.streak.lock().unwrap().clone().unwrap();
        assert_eq!(streak.current_length, 3);
        assert_eq!(streak.longest_length, 3);
    }

    // ── activity accumulation ────────────────────────────────

    #[tokio::test]
    async fn activity_accumulates_across_sessions() {
        let uid = uid();
        let repo = Arc::new(MockRepo::default());
        let uc = use_case(repo.clone());

        uc.execute(uid, date(1), dec!(8), 10, 15).await.unwrap();
        uc.execute(uid, date(1), dec!(9), 12, 18).await.unwrap();

        let a = repo
            .activities
            .lock()
            .unwrap()
            .get(&date(1))
            .cloned()
            .unwrap();
        assert_eq!(a.minutes, dec!(17));
        assert_eq!(a.pages, 22);
        assert_eq!(a.xp_earned, 33);
        assert!(a.met_daily_goal);
    }
}
