use async_trait::async_trait;
use chrono::{DateTime, NaiveTime, Utc};
use persistence::PgPool;
use uuid::Uuid;

use crate::application::ports::ProfileRepository;
use crate::domain::{
    profile::{Profile, ProfileVisibility},
    reminder::{DaysOfWeek, Reminder},
    settings::{AppTheme, ReaderTheme, UserSettings},
};

#[derive(Clone)]
pub struct PgProfileRepository {
    pool: PgPool,
}

impl PgProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct ProfileRow {
    user_id: Uuid,
    bio: Option<String>,
    pronouns: Option<String>,
    location: Option<String>,
    banner_url: Option<String>,
    visibility: String,
    favorite_genres: Option<String>,
    reading_since: Option<chrono::NaiveDate>,
    followers_count: i32,
    friends_count: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ProfileRow> for Profile {
    fn from(r: ProfileRow) -> Self {
        Self {
            user_id: r.user_id,
            bio: r.bio,
            pronouns: r.pronouns,
            location: r.location,
            banner_url: r.banner_url,
            visibility: match r.visibility.as_str() {
                "private" => ProfileVisibility::Private,
                _ => ProfileVisibility::Public,
            },
            favorite_genres: r.favorite_genres,
            reading_since: r.reading_since,
            followers_count: r.followers_count,
            friends_count: r.friends_count,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct UserSettingsRow {
    user_id: Uuid,
    app_theme: String,
    reader_theme: String,
    reader_font_family: Option<String>,
    reader_font_size: i16,
    default_voice_id: Option<Uuid>,
    default_speed: f32,
    default_pitch: f32,
    sleep_timer_minutes: Option<i16>,
    daily_goal_minutes: i16,
    activity_sharing: bool,
    contact_matching_opt_in: bool,
    updated_at: DateTime<Utc>,
}

impl From<UserSettingsRow> for UserSettings {
    fn from(r: UserSettingsRow) -> Self {
        Self {
            user_id: r.user_id,
            app_theme: match r.app_theme.as_str() {
                "dark" => AppTheme::Dark,
                _ => AppTheme::Light,
            },
            reader_theme: match r.reader_theme.as_str() {
                "dark" => ReaderTheme::Dark,
                _ => ReaderTheme::Light,
            },
            reader_font_family: r.reader_font_family,
            reader_font_size: r.reader_font_size,
            default_voice_id: r.default_voice_id,
            default_speed: r.default_speed,
            default_pitch: r.default_pitch,
            sleep_timer_minutes: r.sleep_timer_minutes,
            daily_goal_minutes: r.daily_goal_minutes,
            activity_sharing: r.activity_sharing,
            contact_matching_opt_in: r.contact_matching_opt_in,
            updated_at: r.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct ReminderRow {
    id: Uuid,
    user_id: Uuid,
    time_local: NaiveTime,
    days_of_week: i16,
    enabled: bool,
    created_at: DateTime<Utc>,
}

impl From<ReminderRow> for Reminder {
    fn from(r: ReminderRow) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            time_local: r.time_local,
            days_of_week: DaysOfWeek::from_bits(r.days_of_week),
            type_: crate::domain::reminder::ReminderType::Daily,
            enabled: r.enabled,
            created_at: r.created_at,
        }
    }
}

#[async_trait]
impl ProfileRepository for PgProfileRepository {
    async fn find_profile(&self, user_id: Uuid) -> anyhow::Result<Option<Profile>> {
        let row = sqlx::query_as::<_, ProfileRow>(
            "SELECT user_id, bio, pronouns, location, banner_url,
                    visibility::text, favorite_genres, reading_since,
                    followers_count, friends_count, created_at, updated_at
             FROM user_profiles
             WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Profile::from))
    }

    async fn find_public_profile(&self, handle: &str) -> anyhow::Result<Option<Profile>> {
        let row = sqlx::query_as::<_, ProfileRow>(
            "SELECT p.user_id, p.bio, p.pronouns, p.location, p.banner_url,
                    p.visibility::text, p.favorite_genres, p.reading_since,
                    p.followers_count, p.friends_count, p.created_at, p.updated_at
             FROM user_profiles p
             JOIN users u ON u.id = p.user_id
             WHERE u.handle = $1
               AND p.visibility = 'public'::profile_visibility",
        )
        .bind(handle)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Profile::from))
    }

    async fn upsert_profile(&self, profile: &Profile) -> anyhow::Result<()> {
        let visibility = match profile.visibility {
            ProfileVisibility::Public => "public",
            ProfileVisibility::Private => "private",
        };

        sqlx::query(
            "INSERT INTO user_profiles
                (user_id, bio, pronouns, location, banner_url,
                 visibility, favorite_genres, reading_since, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6::profile_visibility, $7, $8, now())
             ON CONFLICT (user_id) DO UPDATE SET
                bio             = EXCLUDED.bio,
                pronouns        = EXCLUDED.pronouns,
                location        = EXCLUDED.location,
                banner_url      = EXCLUDED.banner_url,
                visibility      = EXCLUDED.visibility,
                favorite_genres = EXCLUDED.favorite_genres,
                reading_since   = EXCLUDED.reading_since,
                updated_at      = now()",
        )
        .bind(profile.user_id)
        .bind(&profile.bio)
        .bind(&profile.pronouns)
        .bind(&profile.location)
        .bind(&profile.banner_url)
        .bind(visibility)
        .bind(&profile.favorite_genres)
        .bind(profile.reading_since)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_profile_by_handle(&self, handle: &str) -> anyhow::Result<Option<Profile>> {
        let row = sqlx::query_as::<_, ProfileRow>(
            "SELECT p.user_id, p.bio, p.pronouns, p.location, p.banner_url,
                    p.visibility::text, p.favorite_genres, p.reading_since,
                    p.followers_count, p.friends_count, p.created_at, p.updated_at
             FROM user_profiles p
             JOIN users u ON u.id = p.user_id
             WHERE u.handle = $1",
        )
        .bind(handle)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Profile::from))
    }

    async fn delete_profile(&self, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM user_profiles WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_settings(&self, user_id: Uuid) -> anyhow::Result<Option<UserSettings>> {
        let row = sqlx::query_as::<_, UserSettingsRow>(
            "SELECT user_id, app_theme::text, reader_theme::text,
                    reader_font_family, reader_font_size, default_voice_id,
                    default_speed::float4, default_pitch::float4, sleep_timer_minutes,
                    daily_goal_minutes, activity_sharing, contact_matching_opt_in, updated_at
             FROM user_settings
             WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(UserSettings::from))
    }

    async fn upsert_settings(&self, settings: &UserSettings) -> anyhow::Result<()> {
        let app_theme = match settings.app_theme {
            AppTheme::Light => "light",
            AppTheme::Dark => "dark",
        };
        let reader_theme = match settings.reader_theme {
            ReaderTheme::Light => "light",
            ReaderTheme::Dark => "dark",
        };

        sqlx::query(
            "INSERT INTO user_settings
                (user_id, app_theme, reader_theme, reader_font_family, reader_font_size,
                 default_voice_id, default_speed, default_pitch, sleep_timer_minutes,
                 daily_goal_minutes, activity_sharing, contact_matching_opt_in, updated_at)
             VALUES ($1, $2::app_theme, $3::reader_theme, $4, $5, $6, $7, $8, $9, $10, $11, $12, now())
             ON CONFLICT (user_id) DO UPDATE SET
                app_theme               = EXCLUDED.app_theme,
                reader_theme            = EXCLUDED.reader_theme,
                reader_font_family      = EXCLUDED.reader_font_family,
                reader_font_size        = EXCLUDED.reader_font_size,
                default_voice_id        = EXCLUDED.default_voice_id,
                default_speed           = EXCLUDED.default_speed,
                default_pitch           = EXCLUDED.default_pitch,
                sleep_timer_minutes     = EXCLUDED.sleep_timer_minutes,
                daily_goal_minutes      = EXCLUDED.daily_goal_minutes,
                activity_sharing        = EXCLUDED.activity_sharing,
                contact_matching_opt_in = EXCLUDED.contact_matching_opt_in,
                updated_at              = now()",
        )
        .bind(settings.user_id)
        .bind(app_theme)
        .bind(reader_theme)
        .bind(&settings.reader_font_family)
        .bind(settings.reader_font_size)
        .bind(settings.default_voice_id)
        .bind(settings.default_speed)
        .bind(settings.default_pitch)
        .bind(settings.sleep_timer_minutes)
        .bind(settings.daily_goal_minutes)
        .bind(settings.activity_sharing)
        .bind(settings.contact_matching_opt_in)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list_reminders(&self, user_id: Uuid) -> anyhow::Result<Vec<Reminder>> {
        let rows = sqlx::query_as::<_, ReminderRow>(
            "SELECT id, user_id, time_local, days_of_week, enabled, created_at
             FROM reminders
             WHERE user_id = $1
             ORDER BY time_local",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Reminder::from).collect())
    }

    async fn create_reminder(&self, reminder: &Reminder) -> anyhow::Result<Uuid> {
        let id = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO reminders (id, user_id, time_local, days_of_week, type, enabled)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id",
        )
        .bind(reminder.id)
        .bind(reminder.user_id)
        .bind(reminder.time_local)
        .bind(reminder.days_of_week.to_bits())
        .bind("reading")
        .bind(reminder.enabled)
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    async fn update_reminder(&self, reminder: &Reminder) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE reminders
             SET time_local = $1, days_of_week = $2, enabled = $3
             WHERE id = $4 AND user_id = $5",
        )
        .bind(reminder.time_local)
        .bind(reminder.days_of_week.to_bits())
        .bind(reminder.enabled)
        .bind(reminder.id)
        .bind(reminder.user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_reminder(&self, reminder_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM reminders WHERE id = $1 AND user_id = $2")
            .bind(reminder_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
