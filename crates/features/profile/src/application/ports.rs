use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    profile::Profile,
    reminder::Reminder,
    settings::UserSettings,
};


#[async_trait]
pub trait ProfileRepository: Send + Sync {
    // profile             
    async fn find_profile(&self, user_id: Uuid) -> anyhow::Result<Option<Profile>>;
    async fn find_public_profile(&self, handle: &str) -> anyhow::Result<Option<Profile>>;
    async fn upsert_profile(&self, profile: &Profile) -> anyhow::Result<()>;
    async fn find_profile_by_handle(&self, handle: &str) -> anyhow::Result<Option<Profile>>;
    async fn delete_profile(&self, user_id: Uuid) -> anyhow::Result<()>;

    // settings   
    async fn find_settings(&self, user_id: Uuid) -> anyhow::Result<Option<UserSettings>>;
    async fn upsert_settings(&self, settings: &UserSettings) -> anyhow::Result<()>;

    // reminders        
    async fn list_reminders(&self, user_id: Uuid) -> anyhow::Result<Vec<Reminder>>;
    async fn create_reminder(&self, reminder: &Reminder) -> anyhow::Result<Uuid>;
    async fn update_reminder(&self, reminder: &Reminder) -> anyhow::Result<()>;
    async fn delete_reminder(&self, reminder_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;
}                                             
      