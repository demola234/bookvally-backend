use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

/// Per-user per-day rollup. Source of truth for streak advancement.
#[derive(Debug, Clone, PartialEq)]
pub struct DailyActivity {
    pub user_id: Uuid,
    pub activity_date: NaiveDate,
    pub minutes: Decimal,
    pub pages: i32,
    pub xp_earned: i32,
    pub met_daily_goal: bool,
}

impl DailyActivity {
    pub fn new(user_id: Uuid, activity_date: NaiveDate) -> Self {
        Self {
            user_id,
            activity_date,
            minutes: Decimal::ZERO,
            pages: 0,
            xp_earned: 0,
            met_daily_goal: false,
        }
    }

    /// Accumulate a reading session into this day's totals.
    /// Returns whether the daily goal was *newly* met (false if already met).
    pub fn accumulate(
        &mut self,
        minutes: Decimal,
        pages: i32,
        xp: i32,
        goal_minutes: Decimal,
    ) -> bool {
        let was_met = self.met_daily_goal;
        self.minutes += minutes;
        self.pages += pages;
        self.xp_earned += xp;
        self.met_daily_goal = self.minutes >= goal_minutes;
        !was_met && self.met_daily_goal // true only on the crossing event
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 6, 16).unwrap()
    }
    fn uid() -> Uuid {
        Uuid::new_v4()
    }
    fn goal() -> Decimal {
        dec!(15)
    } // 15 minutes

    // ── accumulate ───────────────────────────────────────────

    #[test]
    fn accumulate_adds_minutes_and_pages() {
        let mut a = DailyActivity::new(uid(), today());
        a.accumulate(dec!(5), 3, 10, goal());
        a.accumulate(dec!(7), 5, 14, goal());
        assert_eq!(a.minutes, dec!(12));
        assert_eq!(a.pages, 8);
        assert_eq!(a.xp_earned, 24);
    }

    #[test]
    fn goal_not_met_below_threshold() {
        let mut a = DailyActivity::new(uid(), today());
        let newly_met = a.accumulate(dec!(14.99), 0, 0, goal());
        assert!(!a.met_daily_goal);
        assert!(!newly_met);
    }

    #[test]
    fn goal_met_exactly_at_threshold() {
        let mut a = DailyActivity::new(uid(), today());
        let newly_met = a.accumulate(dec!(15), 0, 0, goal());
        assert!(a.met_daily_goal);
        assert!(newly_met);
    }

    #[test]
    fn goal_met_above_threshold() {
        let mut a = DailyActivity::new(uid(), today());
        a.accumulate(dec!(30), 50, 100, goal());
        assert!(a.met_daily_goal);
    }

    #[test]
    fn accumulate_returns_newly_met_only_on_crossing() {
        // First session doesn't cross threshold, second does.
        let mut a = DailyActivity::new(uid(), today());
        let first = a.accumulate(dec!(10), 0, 0, goal());
        let second = a.accumulate(dec!(6), 0, 0, goal()); // now 16 >= 15
        let third = a.accumulate(dec!(5), 0, 0, goal()); // already met
        assert!(!first, "10 min is not enough to meet goal");
        assert!(second, "crossing the threshold should return true once");
        assert!(!third, "already met — no new crossing event");
    }

    #[test]
    fn accumulate_with_zero_goal_always_met() {
        // Edge: goal_minutes = 0 (hypothetical config)
        let mut a = DailyActivity::new(uid(), today());
        let newly_met = a.accumulate(dec!(0), 0, 0, dec!(0));
        assert!(a.met_daily_goal);
        assert!(newly_met);
    }

    #[test]
    fn fresh_activity_has_zero_state() {
        let a = DailyActivity::new(uid(), today());
        assert_eq!(a.minutes, Decimal::ZERO);
        assert_eq!(a.pages, 0);
        assert!(!a.met_daily_goal);
    }
}
