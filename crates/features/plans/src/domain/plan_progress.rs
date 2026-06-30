use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Completed,
    Paused,
    Abandoned,
}

impl SubscriptionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Paused => "paused",
            Self::Abandoned => "abandoned",
        }
    }
}

impl TryFrom<String> for SubscriptionStatus {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "paused" => Ok(Self::Paused),
            "abandoned" => Ok(Self::Abandoned),
            other => Err(anyhow::anyhow!("unknown subscription_status: {other}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanSubscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_id: Uuid,
    pub group_id: Option<Uuid>,
    pub started_on: NaiveDate,
    pub current_day: i32,
    pub status: SubscriptionStatus,
    pub created_at: DateTime<Utc>,
}

impl PlanSubscription {
    pub fn new(user_id: Uuid, plan_id: Uuid, group_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            plan_id,
            group_id,
            started_on: Utc::now().date_naive(),
            current_day: 1,
            status: SubscriptionStatus::Active,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanProgress {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub plan_day_id: Uuid,
    pub completed_at: Option<DateTime<Utc>>,
}

impl PlanProgress {
    pub fn new(subscription_id: Uuid, plan_day_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            subscription_id,
            plan_day_id,
            completed_at: None,
        }
    }
}
