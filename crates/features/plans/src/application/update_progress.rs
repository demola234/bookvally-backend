use kernel::AppError;
use uuid::Uuid;

use crate::application::ports::PlansRepository;
use crate::domain::{PlanProgress, SubscriptionStatus};

pub struct UpdateProgress<R> {
    pub repository: R,
}

impl<R: PlansRepository> UpdateProgress<R> {
    pub async fn execute(
        &self,
        user_id: Uuid,
        subscription_id: Uuid,
        plan_day_id: Uuid,
    ) -> Result<PlanProgress, AppError> {
        let sub = self
            .repository
            .get_subscription_by_id(subscription_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("subscription"))?;

        if sub.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        let plan_detail = self
            .repository
            .get_plan(sub.plan_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("plan"))?;

        let progress = self
            .repository
            .mark_day_complete(subscription_id, plan_day_id)
            .await
            .map_err(AppError::internal)?;

        let completed = self
            .repository
            .count_completed_days(subscription_id)
            .await
            .map_err(AppError::internal)?;

        if completed >= plan_detail.plan.duration_days as i64 {
            self.repository
                .update_subscription_status(subscription_id, SubscriptionStatus::Completed)
                .await
                .map_err(AppError::internal)?;
        } else {
            self.repository
                .advance_current_day(subscription_id, completed as i32 + 1)
                .await
                .map_err(AppError::internal)?;
        }

        Ok(progress)
    }
}
