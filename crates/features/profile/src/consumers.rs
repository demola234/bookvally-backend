use uuid::Uuid;
use tracing::{info};

use crate::adapters::PgProfileRepository;
use crate::application::ports::ProfileRepository;
use crate::domain::profile::Profile;
use crate::domain::settings::UserSettings;

pub struct ProfileEventConsumer {
    pub repository: PgProfileRepository,
}

impl ProfileEventConsumer {
    pub async fn handle_user_registered(&self, user_id: Uuid, handle: String) -> anyhow::Result<()> {
        if self.repository.find_profile(user_id).await?.is_some() {
            info!("profile already exists for user {user_id}, skipping");
            return Ok(());
        }

        self.repository.upsert_profile(&Profile::new(user_id)).await?;
        self.repository.upsert_settings(&UserSettings::new(user_id)).await?;

        info!("created default profile and settings for user {user_id} ({handle})");
        Ok(())
    }
}
