use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::{
    profile::{Profile, ProfileVisibility},
    reminder::Reminder,
    settings::UserSettings,
};

#[derive(Deserialize, ToSchema)]
pub struct UpdateProfileRequest {
    pub bio:             Option<String>,
    pub pronouns:        Option<String>,
    pub location:        Option<String>,
    pub banner_url:      Option<String>,
    pub visibility:      Option<String>,
    pub favorite_genres: Option<String>,
    pub reading_since:   Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSettingsRequest {
    pub app_theme:               Option<String>,
    pub reader_theme:            Option<String>,
    pub reader_font_family:      Option<String>,
    pub reader_font_size:        Option<i16>,
    pub default_speed:           Option<f32>,
    pub default_pitch:           Option<f32>,
    pub sleep_timer_minutes:     Option<i16>,
    pub daily_goal_minutes:      Option<i16>,
    pub activity_sharing:        Option<bool>,
    pub contact_matching_opt_in: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateReminderRequest {
    pub time_local:    String,
    pub days_of_week:  i16,
    pub reminder_type: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateReminderRequest {
    pub time_local:   Option<String>,
    pub days_of_week: Option<i16>,
    pub enabled:      Option<bool>,
}

// ── Responses ────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
pub struct ProfileResponse {
    pub user_id:         Uuid,
    pub bio:             Option<String>,
    pub pronouns:        Option<String>,
    pub location:        Option<String>,
    pub banner_url:      Option<String>,
    pub visibility:      String,
    pub favorite_genres: Option<String>,
    pub reading_since:   Option<String>,
    pub followers_count: i32,
    pub friends_count:   i32,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}

impl From<Profile> for ProfileResponse {
    fn from(p: Profile) -> Self {
        Self {
            user_id:         p.user_id,
            bio:             p.bio,
            pronouns:        p.pronouns,
            location:        p.location,
            banner_url:      p.banner_url,
            visibility:      match p.visibility {
                ProfileVisibility::Public  => "public".into(),
                ProfileVisibility::Private => "private".into(),
            },
            favorite_genres: p.favorite_genres,
            reading_since:   p.reading_since.map(|d| d.to_string()),
            followers_count: p.followers_count,
            friends_count:   p.friends_count,
            created_at:      p.created_at,
            updated_at:      p.updated_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct SettingsResponse {
    pub user_id:                 Uuid,
    pub app_theme:               String,
    pub reader_theme:            String,
    pub reader_font_family:      Option<String>,
    pub reader_font_size:        i16,
    pub default_speed:           f32,
    pub default_pitch:           f32,
    pub sleep_timer_minutes:     Option<i16>,
    pub daily_goal_minutes:      i16,
    pub activity_sharing:        bool,
    pub contact_matching_opt_in: bool,
    pub updated_at:              DateTime<Utc>,
}

impl From<UserSettings> for SettingsResponse {
    fn from(s: UserSettings) -> Self {
        use crate::domain::settings::{AppTheme, ReaderTheme};
        Self {
            user_id:                 s.user_id,
            app_theme:               match s.app_theme {
                AppTheme::Light => "light".into(),
                AppTheme::Dark  => "dark".into(),
            },
            reader_theme:            match s.reader_theme {
                ReaderTheme::Light => "light".into(),
                ReaderTheme::Dark  => "dark".into(),
            },
            reader_font_family:      s.reader_font_family,
            reader_font_size:        s.reader_font_size,
            default_speed:           s.default_speed,
            default_pitch:           s.default_pitch,
            sleep_timer_minutes:     s.sleep_timer_minutes,
            daily_goal_minutes:      s.daily_goal_minutes,
            activity_sharing:        s.activity_sharing,
            contact_matching_opt_in: s.contact_matching_opt_in,
            updated_at:              s.updated_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ReminderResponse {
    pub id:           Uuid,
    pub user_id:      Uuid,
    pub time_local:   String,
    pub days_of_week: i16,
    pub enabled:      bool,
    pub created_at:   DateTime<Utc>,
}

impl From<Reminder> for ReminderResponse {
    fn from(r: Reminder) -> Self {
        Self {
            id:           r.id,
            user_id:      r.user_id,
            time_local:   r.time_local.format("%H:%M").to_string(),
            days_of_week: r.days_of_week.to_bits(),
            enabled:      r.enabled,
            created_at:   r.created_at,
        }
    }
}
