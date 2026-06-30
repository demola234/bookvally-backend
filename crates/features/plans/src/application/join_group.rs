use kernel::AppError;
use uuid::Uuid;

use crate::application::ports::PlansRepository;
use crate::domain::PlanGroupMember;

pub struct JoinGroup<R> {
    pub repository: R,
}

impl<R: PlansRepository> JoinGroup<R> {
    pub async fn execute(
        &self,
        invite_code: &str,
        user_id: Uuid,
    ) -> Result<PlanGroupMember, AppError> {
        let group = self
            .repository
            .find_group_by_invite_code(invite_code)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("plan group"))?;

        let member = self
            .repository
            .join_group(group.id, user_id)
            .await
            .map_err(AppError::internal)?;

        if self
            .repository
            .get_subscription(user_id, group.plan_id)
            .await
            .map_err(AppError::internal)?
            .is_none()
        {
            self.repository
                .subscribe(user_id, group.plan_id, Some(group.id))
                .await
                .map_err(AppError::internal)?;
        }

        Ok(member)
    }
}
