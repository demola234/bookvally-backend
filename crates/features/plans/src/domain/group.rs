use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanGroup {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub owner_id: Uuid,
    pub name: Option<String>,
    pub invite_code: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl PlanGroup {
    pub fn new(plan_id: Uuid, owner_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            plan_id,
            owner_id,
            name: None,
            invite_code: None,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanGroupMember {
    pub id: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

impl PlanGroupMember {
    pub fn new(group_id: Uuid, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            group_id,
            user_id,
            joined_at: Utc::now(),
        }
    }
}
