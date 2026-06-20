use chrono::{NaiveTime, Utc};
use uuid::Uuid;
use kernel::AppError;
use crate::application::ports::ProfileRepository;
use crate::domain::reminder::{DaysOfWeek, Reminder, ReminderType};

pub struct ListReminders<R> { pub repository: R }

impl<R: ProfileRepository> ListReminders<R> {
    pub async fn execute(&self, user_id: Uuid) -> Result<Vec<Reminder>, AppError> {
        self.repository
            .list_reminders(user_id)
            .await
            .map_err(AppError::internal)
    }
}

pub struct CreateReminderInput {
    pub time_local:    NaiveTime,
    pub days_of_week:  i16,
    pub reminder_type: String,
}

pub struct CreateReminder<R> { pub repository: R }

impl<R: ProfileRepository> CreateReminder<R> {
    pub async fn execute(&self, user_id: Uuid, input: CreateReminderInput) -> Result<Uuid, AppError> {
        let reminder = Reminder {
            id:           Uuid::new_v4(),
            user_id,
            time_local:   input.time_local,
            days_of_week: DaysOfWeek::from_bits(input.days_of_week),
            type_:        ReminderType::Daily,
            enabled:      true,
            created_at:   Utc::now(),
        };

        self.repository
            .create_reminder(&reminder)
            .await
            .map_err(AppError::internal)
    }
}

pub struct UpdateReminderInput {
    pub reminder_id:  Uuid,
    pub time_local:   Option<NaiveTime>,
    pub days_of_week: Option<i16>,
    pub enabled:      Option<bool>,
}

pub struct UpdateReminder<R> { pub repository: R }

impl<R: ProfileRepository> UpdateReminder<R> {
    pub async fn execute(&self, user_id: Uuid, input: UpdateReminderInput) -> Result<(), AppError> {
        let mut reminders = self.repository
            .list_reminders(user_id)
            .await
            .map_err(AppError::internal)?;

        let reminder = reminders
            .iter_mut()
            .find(|r| r.id == input.reminder_id)
            .ok_or_else(|| AppError::not_found("reminder"))?;

        if let Some(t) = input.time_local    { reminder.time_local   = t; }
        if let Some(d) = input.days_of_week  { reminder.days_of_week = DaysOfWeek::from_bits(d); }
        if let Some(e) = input.enabled       { reminder.enabled      = e; }

        self.repository
            .update_reminder(reminder)
            .await
            .map_err(AppError::internal)
    }
}

pub struct DeleteReminder<R> { pub repository: R }

impl<R: ProfileRepository> DeleteReminder<R> {
    pub async fn execute(&self, user_id: Uuid, reminder_id: Uuid) -> Result<(), AppError> {
        self.repository
            .delete_reminder(reminder_id, user_id)
            .await
            .map_err(AppError::internal)
    }
}
