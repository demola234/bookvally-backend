use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    PlanDay, PlanDayItem, PlanDetail, PlanGroup, PlanGroupMember, PlanProgress, PlanSubscription,
    ReadingPlan, SubscriptionStatus,
};

#[async_trait]
pub trait PlansRepository: Send + Sync + Clone {
    async fn list_plans(
        &self,
        category: Option<String>,
        official_only: bool,
    ) -> anyhow::Result<Vec<ReadingPlan>>;

    async fn get_plan(&self, id: Uuid) -> anyhow::Result<Option<PlanDetail>>;

    async fn create_plan(
        &self,
        plan: ReadingPlan,
        days: Vec<(PlanDay, Vec<PlanDayItem>)>,
    ) -> anyhow::Result<PlanDetail>;

    async fn subscribe(
        &self,
        user_id: Uuid,
        plan_id: Uuid,
        group_id: Option<Uuid>,
    ) -> anyhow::Result<PlanSubscription>;

    async fn get_subscription(
        &self,
        user_id: Uuid,
        plan_id: Uuid,
    ) -> anyhow::Result<Option<PlanSubscription>>;

    async fn get_subscription_by_id(
        &self,
        subscription_id: Uuid,
    ) -> anyhow::Result<Option<PlanSubscription>>;

    async fn list_subscriptions(&self, user_id: Uuid) -> anyhow::Result<Vec<PlanSubscription>>;

    async fn update_subscription_status(
        &self,
        subscription_id: Uuid,
        status: SubscriptionStatus,
    ) -> anyhow::Result<()>;

    async fn advance_current_day(&self, subscription_id: Uuid, day: i32) -> anyhow::Result<()>;

    async fn mark_day_complete(
        &self,
        subscription_id: Uuid,
        plan_day_id: Uuid,
    ) -> anyhow::Result<PlanProgress>;

    async fn get_progress(&self, subscription_id: Uuid) -> anyhow::Result<Vec<PlanProgress>>;

    async fn count_completed_days(&self, subscription_id: Uuid) -> anyhow::Result<i64>;

    async fn create_group(
        &self,
        plan_id: Uuid,
        owner_id: Uuid,
        name: Option<String>,
    ) -> anyhow::Result<PlanGroup>;

    async fn find_group_by_invite_code(
        &self,
        invite_code: &str,
    ) -> anyhow::Result<Option<PlanGroup>>;

    async fn join_group(&self, group_id: Uuid, user_id: Uuid) -> anyhow::Result<PlanGroupMember>;

    async fn list_group_members(&self, group_id: Uuid) -> anyhow::Result<Vec<PlanGroupMember>>;
}
