use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::{
    PlanDay, PlanDayItem, PlanDetail, PlanGroup, PlanGroupMember, PlanProgress, PlanSubscription,
    ReadingPlan,
};

// ── Query params ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListPlansQuery {
    pub category: Option<String>,
    pub official_only: Option<bool>,
}

// ── Request bodies ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDayItemRequest {
    pub book_id: Option<Uuid>,
    pub from_locator: Option<String>,
    pub to_locator: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDayRequest {
    pub day_number: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub items: Vec<CreateDayItemRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePlanRequest {
    pub title: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub category: Option<String>,
    pub duration_days: i32,
    pub days: Vec<CreateDayRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubscribeRequest {
    pub group_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarkDayRequest {
    pub plan_day_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateGroupRequest {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct JoinGroupRequest {
    pub invite_code: String,
}

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanResponse {
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

impl From<ReadingPlan> for PlanResponse {
    fn from(p: ReadingPlan) -> Self {
        Self {
            id: p.id,
            title: p.title,
            description: p.description,
            cover_url: p.cover_url,
            category: p.category,
            duration_days: p.duration_days,
            is_official: p.is_official,
            creator_id: p.creator_id,
            created_at: p.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanDayItemResponse {
    pub id: Uuid,
    pub book_id: Option<Uuid>,
    pub from_locator: Option<String>,
    pub to_locator: Option<String>,
    pub label: Option<String>,
}

impl From<PlanDayItem> for PlanDayItemResponse {
    fn from(i: PlanDayItem) -> Self {
        Self {
            id: i.id,
            book_id: i.book_id,
            from_locator: i.from_locator,
            to_locator: i.to_locator,
            label: i.label,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanDayResponse {
    pub id: Uuid,
    pub day_number: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub items: Vec<PlanDayItemResponse>,
}

impl From<(PlanDay, Vec<PlanDayItem>)> for PlanDayResponse {
    fn from((day, items): (PlanDay, Vec<PlanDayItem>)) -> Self {
        Self {
            id: day.id,
            day_number: day.day_number,
            title: day.title,
            description: day.description,
            items: items.into_iter().map(PlanDayItemResponse::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanDetailResponse {
    #[serde(flatten)]
    pub plan: PlanResponse,
    pub days: Vec<PlanDayResponse>,
}

impl From<PlanDetail> for PlanDetailResponse {
    fn from(d: PlanDetail) -> Self {
        Self {
            plan: PlanResponse::from(d.plan),
            days: d.days.into_iter().map(PlanDayResponse::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionResponse {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub group_id: Option<Uuid>,
    pub started_on: NaiveDate,
    pub current_day: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<PlanSubscription> for SubscriptionResponse {
    fn from(s: PlanSubscription) -> Self {
        Self {
            id: s.id,
            plan_id: s.plan_id,
            group_id: s.group_id,
            started_on: s.started_on,
            current_day: s.current_day,
            status: s.status.as_str().to_string(),
            created_at: s.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProgressResponse {
    pub id: Uuid,
    pub plan_day_id: Uuid,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<PlanProgress> for ProgressResponse {
    fn from(p: PlanProgress) -> Self {
        Self {
            id: p.id,
            plan_day_id: p.plan_day_id,
            completed_at: p.completed_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GroupResponse {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub owner_id: Uuid,
    pub name: Option<String>,
    pub invite_code: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<PlanGroup> for GroupResponse {
    fn from(g: PlanGroup) -> Self {
        Self {
            id: g.id,
            plan_id: g.plan_id,
            owner_id: g.owner_id,
            name: g.name,
            invite_code: g.invite_code,
            created_at: g.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GroupMemberResponse {
    pub id: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

impl From<PlanGroupMember> for GroupMemberResponse {
    fn from(m: PlanGroupMember) -> Self {
        Self {
            id: m.id,
            group_id: m.group_id,
            user_id: m.user_id,
            joined_at: m.joined_at,
        }
    }
}
