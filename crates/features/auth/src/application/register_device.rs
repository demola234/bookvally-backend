use kernel::{AppError, UserId};
use chrono::Utc;
use uuid::Uuid;

use crate::application::ports::AuthRepository;
use crate::domain::{Device};

pub struct RegisterDevice<R> {
    pub repository: R,
}

impl<R: AuthRepository> RegisterDevice<R> {
    pub async fn execute(
        &self,
        user_id: UserId,
        platform: String,
        device_name: Option<String>,
        push_token: Option<String>,
        app_version: Option<String>,
    ) -> Result<Uuid, AppError> {

        let device = Device {
            id: Uuid::new_v4(),
            user_id,
            platform,
            device_name,
            push_token,
            app_version,
            last_seen_at:  Some(Utc::now())
        };

        let id = self.repository.upsert_device(&device).await.map_err(AppError::internal)?;
        Ok(id)
    }
}