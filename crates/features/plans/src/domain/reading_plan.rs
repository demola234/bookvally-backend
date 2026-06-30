use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ReadingPlan {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub category: Option<String>,
    pub duration_days: i32,
    pub is_official: bool,
    pub creator_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl ReadingPlan {
    pub fn new(
        title: String,
        description: Option<String>,
        cover_url: Option<String>,
        category: Option<String>,
        duration_days: i32,
        creator_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            cover_url,
            category,
            duration_days,
            is_official: false,
            creator_id: Some(creator_id),
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanDay {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub day_number: i32,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl PlanDay {
    pub fn new(
        plan_id: Uuid,
        day_number: i32,
        title: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            plan_id,
            day_number,
            title,
            description,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanDayItem {
    pub id: Uuid,
    pub plan_day_id: Uuid,
    pub book_id: Option<Uuid>,
    pub from_locator: Option<String>,
    pub to_locator: Option<String>,
    pub label: Option<String>,
}

impl PlanDayItem {
    pub fn new(
        plan_day_id: Uuid,
        book_id: Option<Uuid>,
        from_locator: Option<String>,
        to_locator: Option<String>,
        label: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            plan_day_id,
            book_id,
            from_locator,
            to_locator,
            label,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanDetail {
    pub plan: ReadingPlan,
    pub days: Vec<(PlanDay, Vec<PlanDayItem>)>,
}
