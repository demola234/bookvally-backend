use kernel::AppError;
use uuid::Uuid;

use crate::application::ports::PlansRepository;
use crate::domain::{PlanDay, PlanDayItem, PlanDetail, ReadingPlan};

pub struct CreateItemInput {
    pub book_id: Option<Uuid>,
    pub from_locator: Option<String>,
    pub to_locator: Option<String>,
    pub label: Option<String>,
}

pub struct CreateDayInput {
    pub day_number: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub items: Vec<CreateItemInput>,
}

pub struct CreatePlanInput {
    pub title: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub category: Option<String>,
    pub duration_days: i32,
    pub creator_id: Uuid,
    pub days: Vec<CreateDayInput>,
}

pub struct CreatePlan<R> {
    pub repository: R,
}

impl<R: PlansRepository> CreatePlan<R> {
    pub async fn execute(&self, input: CreatePlanInput) -> Result<PlanDetail, AppError> {
        if input.days.len() != input.duration_days as usize {
            return Err(AppError::UnprocessableEntity(format!(
                "duration_days is {} but {} days were provided",
                input.duration_days,
                input.days.len(),
            )));
        }

        let plan = ReadingPlan::new(
            input.title,
            input.description,
            input.cover_url,
            input.category,
            input.duration_days,
            input.creator_id,
        );

        let days = input
            .days
            .into_iter()
            .map(|d| {
                let day = PlanDay::new(plan.id, d.day_number, d.title, d.description);
                let items = d
                    .items
                    .into_iter()
                    .map(|i| {
                        PlanDayItem::new(day.id, i.book_id, i.from_locator, i.to_locator, i.label)
                    })
                    .collect();
                (day, items)
            })
            .collect();

        self.repository
            .create_plan(plan, days)
            .await
            .map_err(AppError::internal)
    }
}
