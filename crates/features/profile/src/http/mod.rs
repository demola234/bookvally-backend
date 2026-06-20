pub mod dto;
pub mod routes;

use utoipa::OpenApi;
use dto::*;
use routes::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_profile, update_profile, get_public_profile,
        get_settings, update_settings,
        list_reminders, create_reminder, update_reminder, delete_reminder,
    ),
    components(schemas(
        ProfileResponse, UpdateProfileRequest,
        SettingsResponse, UpdateSettingsRequest,
        ReminderResponse, CreateReminderRequest, UpdateReminderRequest,
    ))
)]
pub struct ProfileApiDoc;