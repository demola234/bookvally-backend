use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::{get, patch}};
use auth_kit::JwtAuthExtractor;
use chrono::NaiveTime;
use http_kit::{v1, HttpError};
use kernel::AppError;
use uuid::Uuid;

use crate::application::{
    get_profile::{GetProfile, GetPublicProfile},
    ports::ProfileRepository,
    set_reminder::{CreateReminder, CreateReminderInput, DeleteReminder, ListReminders, UpdateReminder, UpdateReminderInput},
    update_profile::{UpdateProfile, UpdateProfileInput},
    update_settings::{UpdateSettings, UpdateSettingsInput},
};
use crate::http::dto::*;
use crate::wiring::ProfileState;

pub fn routes() -> Router<ProfileState> {
    v1(Router::new()
        .route("/profile",               get(get_profile).patch(update_profile))
        .route("/profile/{handle}",       get(get_public_profile))
        .route("/profile/settings",      get(get_settings).patch(update_settings))
        .route("/profile/reminders",     get(list_reminders).post(create_reminder))
        .route("/profile/reminders/{id}", patch(update_reminder).delete(delete_reminder))
    )
}

#[utoipa::path(get, path = "/v1/profile", tag = "profile",
    security(("bearer_auth" = [])),
    responses((status = 200, body = ProfileResponse), (status = 401, description = "Unauthorized"))
)]
pub async fn get_profile(
    State(state): State<ProfileState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
) -> Result<Json<ProfileResponse>, HttpError> {
    let profile = GetProfile { repository: state.repo }
        .execute(*user.id().as_uuid())
        .await
        .map_err(HttpError::from)?;
    Ok(Json(ProfileResponse::from(profile)))
}

#[utoipa::path(patch, path = "/v1/profile", tag = "profile",
    request_body = UpdateProfileRequest,
    security(("bearer_auth" = [])),
    responses((status = 200, body = ProfileResponse), (status = 401, description = "Unauthorized"))
)]
pub async fn update_profile(
    State(state): State<ProfileState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<Json<ProfileResponse>, HttpError> {
    let profile = UpdateProfile { repository: state.repo }
        .execute(*user.id().as_uuid(), UpdateProfileInput {
            bio:             body.bio,
            pronouns:        body.pronouns,
            location:        body.location,
            banner_url:      body.banner_url,
            visibility:      body.visibility,
            favorite_genres: body.favorite_genres,
            reading_since:   body.reading_since,
        })
        .await
        .map_err(HttpError::from)?;
    Ok(Json(ProfileResponse::from(profile)))
}

#[utoipa::path(get, path = "/v1/profile/{handle}", tag = "profile",
    params(("handle" = String, Path, description = "User handle")),
    responses((status = 200, body = ProfileResponse), (status = 404, description = "Not found"))
)]
pub async fn get_public_profile(
    State(state): State<ProfileState>,
    Path(handle): Path<String>,
) -> Result<Json<ProfileResponse>, HttpError> {
    let profile = GetPublicProfile { repository: state.repo }
        .execute(handle)
        .await
        .map_err(HttpError::from)?;
    Ok(Json(ProfileResponse::from(profile)))
}

#[utoipa::path(get, path = "/v1/profile/settings", tag = "profile",
    security(("bearer_auth" = [])),
    responses((status = 200, body = SettingsResponse), (status = 401, description = "Unauthorized"))
)]
pub async fn get_settings(
    State(state): State<ProfileState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
) -> Result<Json<SettingsResponse>, HttpError> {
    let s = state.repo
        .find_settings(*user.id().as_uuid())
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError::from(AppError::not_found("settings")))?;
    Ok(Json(SettingsResponse::from(s)))
}

#[utoipa::path(patch, path = "/v1/profile/settings", tag = "profile",
    request_body = UpdateSettingsRequest,
    security(("bearer_auth" = [])),
    responses((status = 200, body = SettingsResponse), (status = 401, description = "Unauthorized"))
)]
pub async fn update_settings(
    State(state): State<ProfileState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Json(body): Json<UpdateSettingsRequest>,
) -> Result<Json<SettingsResponse>, HttpError> {
    let s = UpdateSettings { repository: state.repo }
        .execute(*user.id().as_uuid(), UpdateSettingsInput {
            app_theme:               body.app_theme,
            reader_theme:            body.reader_theme,
            reader_font_family:      body.reader_font_family,
            reader_font_size:        body.reader_font_size,
            default_speed:           body.default_speed,
            default_pitch:           body.default_pitch,
            sleep_timer_minutes:     body.sleep_timer_minutes,
            daily_goal_minutes:      body.daily_goal_minutes,
            activity_sharing:        body.activity_sharing,
            contact_matching_opt_in: body.contact_matching_opt_in,
        })
        .await
        .map_err(HttpError::from)?;
    Ok(Json(SettingsResponse::from(s)))
}

#[utoipa::path(get, path = "/v1/profile/reminders", tag = "profile",
    security(("bearer_auth" = [])),
    responses((status = 200, body = Vec<ReminderResponse>), (status = 401, description = "Unauthorized"))
)]
pub async fn list_reminders(
    State(state): State<ProfileState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
) -> Result<Json<Vec<ReminderResponse>>, HttpError> {
    let reminders = ListReminders { repository: state.repo }
        .execute(*user.id().as_uuid())
        .await
        .map_err(HttpError::from)?;
    Ok(Json(reminders.into_iter().map(ReminderResponse::from).collect()))
}

#[utoipa::path(post, path = "/v1/profile/reminders", tag = "profile",
    request_body = CreateReminderRequest,
    security(("bearer_auth" = [])),
    responses((status = 201, description = "Reminder created"), (status = 401, description = "Unauthorized"))
)]
pub async fn create_reminder(
    State(state): State<ProfileState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Json(body): Json<CreateReminderRequest>,
) -> Result<StatusCode, HttpError> {
    let time = NaiveTime::parse_from_str(&body.time_local, "%H:%M")
        .map_err(|_| HttpError::from(AppError::UnprocessableEntity("invalid time format, use HH:MM".into())))?;

    CreateReminder { repository: state.repo }
        .execute(*user.id().as_uuid(), CreateReminderInput {
            time_local:    time,
            days_of_week:  body.days_of_week,
            reminder_type: body.reminder_type.unwrap_or_else(|| "reading".into()),
        })
        .await
        .map_err(HttpError::from)?;

    Ok(StatusCode::CREATED)
}

#[utoipa::path(patch, path = "/v1/profile/reminders/{id}", tag = "profile",
    params(("id" = Uuid, Path, description = "Reminder ID")),
    request_body = UpdateReminderRequest,
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Updated"), (status = 404, description = "Not found"))
)]
pub async fn update_reminder(
    State(state): State<ProfileState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateReminderRequest>,
) -> Result<StatusCode, HttpError> {
    let time = body.time_local.as_deref()
        .map(|t| NaiveTime::parse_from_str(t, "%H:%M")
            .map_err(|_| HttpError::from(AppError::UnprocessableEntity("invalid time format".into()))))
        .transpose()?;

    UpdateReminder { repository: state.repo }
        .execute(*user.id().as_uuid(), UpdateReminderInput {
            reminder_id:  id,
            time_local:   time,
            days_of_week: body.days_of_week,
            enabled:      body.enabled,
        })
        .await
        .map_err(HttpError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/v1/profile/reminders/{id}", tag = "profile",
    params(("id" = Uuid, Path, description = "Reminder ID")),
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Deleted"), (status = 404, description = "Not found"))
)]
pub async fn delete_reminder(
    State(state): State<ProfileState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, HttpError> {
    DeleteReminder { repository: state.repo }
        .execute(*user.id().as_uuid(), id)
        .await
        .map_err(HttpError::from)?;
    Ok(StatusCode::NO_CONTENT)
}
