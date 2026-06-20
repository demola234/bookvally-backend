use chrono::{DateTime, NaiveTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DaysOfWeek {
    pub monday:    bool,
    pub tuesday:   bool,
    pub wednesday: bool,
    pub thursday:  bool,
    pub friday:    bool,
    pub saturday:  bool,
    pub sunday:    bool,
}

impl DaysOfWeek {
    pub fn from_bits(bits: i16) -> Self {
        Self {
            monday:    bits & 1  != 0,
            tuesday:   bits & 2  != 0,
            wednesday: bits & 4  != 0,
            thursday:  bits & 8  != 0,
            friday:    bits & 16 != 0,
            saturday:  bits & 32 != 0,
            sunday:    bits & 64 != 0,
        }
    }

    pub fn to_bits(&self) -> i16 {
          (self.monday    as i16)
        | (self.tuesday   as i16) << 1
        | (self.wednesday as i16) << 2
        | (self.thursday  as i16) << 3
        | (self.friday    as i16) << 4
        | (self.saturday  as i16) << 5
        | (self.sunday    as i16) << 6
    }
}
#[derive(Clone, Debug)]
pub enum ReminderType {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub time_local: NaiveTime,
    pub days_of_week: DaysOfWeek,
    pub type_: ReminderType,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

impl Reminder {
    pub fn is_scheduled_on(&self, day: &DaysOfWeek) -> bool {
        self.days_of_week == *day
    }
}