use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

pub trait Clock: Send + Sync + 'static {
    fn now(&self) -> DateTime<Utc>;
    fn today(&self) -> NaiveDate;
    fn today_with_time(&self) -> NaiveDateTime;

    fn today_for_user(&self, iana_tz: &str) -> NaiveDate;
}

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn today(&self) -> NaiveDate {
        Utc::now().naive_utc().date()
    }

    fn today_with_time(&self) -> NaiveDateTime {
        Utc::now().naive_utc()
    }

    fn today_for_user(&self, iana_tz: &str) -> NaiveDate {
        iana_tz
            .parse::<chrono_tz::Tz>()
            .map(|tz| Utc::now().with_timezone(&tz).date_naive())
            .unwrap_or_else(|_| Utc::now().date_naive())
    }
}

pub struct SystemClock;
