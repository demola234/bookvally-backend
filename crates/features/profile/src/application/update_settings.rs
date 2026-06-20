use uuid::Uuid;
use chrono::Utc;
use kernel::AppError;
use crate::application::ports::ProfileRepository;
use crate::domain::settings::{AppTheme, ReaderTheme, UserSettings};

pub struct UpdateSettingsInput {
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

pub struct UpdateSettings<R> { pub repository: R }

impl<R: ProfileRepository> UpdateSettings<R> {
    pub async fn execute(&self, user_id: Uuid, input: UpdateSettingsInput) -> Result<UserSettings, AppError> {
        let existing = self.repository
            .find_settings(user_id)
            .await
            .map_err(AppError::internal)?
            .unwrap_or_else(|| UserSettings::new(user_id));

        let app_theme = match input.app_theme.as_deref() {
            Some("light") => AppTheme::Light,
            Some("dark")  => AppTheme::Dark,
            Some(other)   => return Err(AppError::UnprocessableEntity(format!("invalid app_theme: {other}"))),
            None          => existing.app_theme,
        };

        let reader_theme = match input.reader_theme.as_deref() {
            Some("light") => ReaderTheme::Light,
            Some("dark")  => ReaderTheme::Dark,
            Some(other)   => return Err(AppError::UnprocessableEntity(format!("invalid reader_theme: {other}"))),
            None          => existing.reader_theme,
        };

        let updated = UserSettings {
            user_id,
            app_theme,
            reader_theme,
            reader_font_family:      input.reader_font_family.or(existing.reader_font_family),
            reader_font_size:        input.reader_font_size.unwrap_or(existing.reader_font_size),
            default_voice_id:        existing.default_voice_id,
            default_speed:           input.default_speed.unwrap_or(existing.default_speed),
            default_pitch:           input.default_pitch.unwrap_or(existing.default_pitch),
            sleep_timer_minutes:     input.sleep_timer_minutes.or(existing.sleep_timer_minutes),
            daily_goal_minutes:      input.daily_goal_minutes.unwrap_or(existing.daily_goal_minutes),
            activity_sharing:        input.activity_sharing.unwrap_or(existing.activity_sharing),
            contact_matching_opt_in: input.contact_matching_opt_in.unwrap_or(existing.contact_matching_opt_in),
            updated_at:              Utc::now(),
        };

        self.repository.upsert_settings(&updated).await.map_err(AppError::internal)?;

        Ok(updated)
    }
}
