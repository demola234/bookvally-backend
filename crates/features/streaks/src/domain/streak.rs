use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Streak {
    pub user_id:          Uuid,
    pub current_length:   i32,
    pub longest_length:   i32,
    pub started_on:       Option<NaiveDate>,
    pub last_extended_on: Option<NaiveDate>,
    pub freezes_equipped: i16,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum StreakError {
    #[error("no freeze available to consume")]
    NoFreezeAvailable,
    #[error("day already processed: {0}")]
    DayAlreadyProcessed(NaiveDate),
}

impl Streak {
    pub const MAX_EQUIPPED_FREEZES: i16 = 2;

    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            current_length: 0,
            longest_length: 0,
            started_on: None,
            last_extended_on: None,
            freezes_equipped: 0,
        }
    }

    /// Advance the streak for `today`. Idempotent if already extended today.
    pub fn advance(&mut self, today: NaiveDate) -> Result<(), StreakError> {
        if self.last_extended_on == Some(today) {
            return Err(StreakError::DayAlreadyProcessed(today));
        }
        self.current_length += 1;
        if self.started_on.is_none() {
            self.started_on = Some(today);
        }
        if self.current_length > self.longest_length {
            self.longest_length = self.current_length;
        }
        self.last_extended_on = Some(today);
        Ok(())
    }

    /// Consume one equipped freeze instead of breaking the streak.
    pub fn consume_freeze(&mut self, missed_day: NaiveDate) -> Result<(), StreakError> {
        if self.freezes_equipped == 0 {
            return Err(StreakError::NoFreezeAvailable);
        }
        self.freezes_equipped -= 1;
        // Mark last_extended_on so the next day doesn't double-penalise.
        self.last_extended_on = Some(missed_day);
        Ok(())
    }

    /// Break the streak — resets current, preserves longest.
    pub fn break_streak(&mut self) {
        self.current_length = 0;
        self.started_on = None;
        self.last_extended_on = None;
    }

    /// Equip a freeze (e.g. from earning a milestone or purchasing).
    /// Returns false if already at max capacity.
    pub fn equip_freeze(&mut self) -> bool {
        if self.freezes_equipped >= Self::MAX_EQUIPPED_FREEZES {
            return false;
        }
        self.freezes_equipped += 1;
        true
    }

    pub fn is_active(&self) -> bool {
        self.current_length > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn user() -> Uuid { Uuid::new_v4() }

    // ── advance ──────────────────────────────────────────────

    #[test]
    fn advance_starts_at_one() {
        let mut s = Streak::new(user());
        s.advance(date(2026, 6, 1)).unwrap();
        assert_eq!(s.current_length, 1);
        assert_eq!(s.longest_length, 1);
        assert_eq!(s.started_on, Some(date(2026, 6, 1)));
    }

    #[test]
    fn advance_increments_current_and_updates_longest() {
        let mut s = Streak::new(user());
        s.advance(date(2026, 6, 1)).unwrap();
        s.advance(date(2026, 6, 2)).unwrap();
        s.advance(date(2026, 6, 3)).unwrap();
        assert_eq!(s.current_length, 3);
        assert_eq!(s.longest_length, 3);
    }

    #[test]
    fn advance_same_day_is_idempotent_error() {
        // Calling advance twice on the same date must not double-count.
        let mut s = Streak::new(user());
        s.advance(date(2026, 6, 1)).unwrap();
        let err = s.advance(date(2026, 6, 1)).unwrap_err();
        assert_eq!(err, StreakError::DayAlreadyProcessed(date(2026, 6, 1)));
        assert_eq!(s.current_length, 1); // unchanged
    }

    #[test]
    fn advance_preserves_longest_after_reset() {
        let mut s = Streak::new(user());
        s.advance(date(2026, 6, 1)).unwrap();
        s.advance(date(2026, 6, 2)).unwrap();
        s.advance(date(2026, 6, 3)).unwrap(); // longest = 3
        s.break_streak();
        s.advance(date(2026, 6, 10)).unwrap(); // current = 1
        assert_eq!(s.current_length, 1);
        assert_eq!(s.longest_length, 3); // must survive break
    }

    #[test]
    fn advance_does_not_update_longest_when_current_below() {
        let mut s = Streak::new(user());
        s.advance(date(2026, 6, 1)).unwrap();
        s.advance(date(2026, 6, 2)).unwrap(); // longest = 2
        s.break_streak();
        s.advance(date(2026, 6, 10)).unwrap(); // current = 1, longest stays 2
        assert_eq!(s.longest_length, 2);
    }

    // ── freeze ───────────────────────────────────────────────

    #[test]
    fn consume_freeze_decrements_count_and_marks_day() {
        let mut s = Streak::new(user());
        s.equip_freeze();
        s.advance(date(2026, 6, 1)).unwrap();
        s.consume_freeze(date(2026, 6, 2)).unwrap();
        assert_eq!(s.freezes_equipped, 0);
        assert_eq!(s.last_extended_on, Some(date(2026, 6, 2)));
        assert_eq!(s.current_length, 1); // streak NOT reset
    }

    #[test]
    fn consume_freeze_with_none_available_is_err() {
        let mut s = Streak::new(user());
        let err = s.consume_freeze(date(2026, 6, 2)).unwrap_err();
        assert_eq!(err, StreakError::NoFreezeAvailable);
    }

    #[test]
    fn equip_freeze_caps_at_max() {
        let mut s = Streak::new(user());
        assert!(s.equip_freeze());
        assert!(s.equip_freeze());
        assert!(!s.equip_freeze()); // third rejected
        assert_eq!(s.freezes_equipped, Streak::MAX_EQUIPPED_FREEZES);
    }

    // ── break ────────────────────────────────────────────────

    #[test]
    fn break_resets_current_and_clears_dates() {
        let mut s = Streak::new(user());
        s.advance(date(2026, 6, 1)).unwrap();
        s.advance(date(2026, 6, 2)).unwrap();
        s.break_streak();
        assert_eq!(s.current_length, 0);
        assert_eq!(s.started_on, None);
        assert_eq!(s.last_extended_on, None);
    }

    #[test]
    fn break_on_fresh_streak_is_noop() {
        let mut s = Streak::new(user());
        s.break_streak(); // must not panic
        assert_eq!(s.current_length, 0);
        assert_eq!(s.longest_length, 0);
    }

    // ── is_active ────────────────────────────────────────────

    #[test]
    fn is_active_false_before_first_advance() {
        assert!(!Streak::new(user()).is_active());
    }

    #[test]
    fn is_active_false_after_break() {
        let mut s = Streak::new(user());
        s.advance(date(2026, 6, 1)).unwrap();
        s.break_streak();
        assert!(!s.is_active());
    }
}
