use crate::application::ports::ProfileRepository;
use crate::domain::profile::Profile;
use kernel::AppError;
use uuid::Uuid;

pub struct GetProfile<R> {
    pub repository: R,
}

impl<R: ProfileRepository> GetProfile<R> {
    pub async fn execute(&self, user_id: Uuid) -> Result<Profile, AppError> {
        self.repository
            .find_profile(user_id)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("profile"))
    }
}

pub struct GetPublicProfile<R> {
    pub repository: R,
}

impl<R: ProfileRepository> GetPublicProfile<R> {
    pub async fn execute(&self, handle: String) -> Result<Profile, AppError> {
        self.repository
            .find_public_profile(&handle)
            .await
            .map_err(AppError::internal)?
            .ok_or_else(|| AppError::not_found("profile"))
    }
}
