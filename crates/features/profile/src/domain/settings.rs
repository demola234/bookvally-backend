use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AppTheme {
    Light,
    Dark,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self::Light
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReaderTheme {
    Light,
    Dark,
}

impl Default for ReaderTheme {
    fn default() -> Self {
        Self::Light
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserSettings {
    pub user_id: Uuid,
    pub app_theme: AppTheme,
    pub reader_theme: ReaderTheme,
    pub reader_font_family: Option<String>,
    pub reader_font_size: i16,
    pub default_voice_id: Option<Uuid>,
    pub default_speed: f32,
    pub default_pitch: f32,
    pub sleep_timer_minutes: Option<i16>,
    pub daily_goal_minutes: i16,
    pub activity_sharing: bool,
    pub contact_matching_opt_in: bool,
    pub updated_at: DateTime<Utc>,
}


impl UserSettings {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            app_theme: AppTheme::default(),
            reader_theme: ReaderTheme::default(),
            reader_font_family: Some("Source Serif 4".to_string()),
            reader_font_size: 16,
            default_voice_id: None,
            default_speed: 1.0,
            default_pitch: 1.0,
            sleep_timer_minutes: Some(30),
            daily_goal_minutes: 15,
            activity_sharing: true,
            contact_matching_opt_in: false,
            updated_at: Utc::now(),
        }
    }
}