use auth_kit::JwtAuthExtractor;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use http_kit::{v1, HttpError};
use kernel::AppError;
use uuid::Uuid;

use crate::application::{
    create_plan::{CreateDayInput, CreateItemInput, CreatePlan, CreatePlanInput},
    join_group::JoinGroup,
    ports::PlansRepository,
    update_progress::UpdateProgress,
};
use crate::http::dto::*;
use crate::wiring::PlansState;

pub fn routes() -> Router<PlansState> {
    v1(Router::new()
        .route("/plans", get(list_plans).post(create_plan))
        .route("/plans/{id}", get(get_plan))
        .route("/plans/{id}/subscribe", post(subscribe))
        .route("/plans/{id}/groups", post(create_group))
        .route("/plans/subscriptions", get(list_subscriptions))
        .route(
            "/plans/subscriptions/{sub_id}/progress",
            get(get_progress).post(mark_day_complete),
        )
        .route("/plans/groups/join", post(join_group)))
}

// ── Catalog ───────────────────────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/v1/plans",
    tag = "plans",
    params(ListPlansQuery),
    responses(
        (status = 200, description = "List of reading plans", body = Vec<PlanResponse>),
    )
)]
pub async fn list_plans(
    State(state): State<PlansState>,
    Query(q): Query<ListPlansQuery>,
) -> Result<Json<Vec<PlanResponse>>, HttpError> {
    let plans = state
        .repo
        .list_plans(q.category, q.official_only.unwrap_or(false))
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok(Json(plans.into_iter().map(PlanResponse::from).collect()))
}

#[utoipa::path(
    post,
    path = "/v1/plans",
    tag = "plans",
    request_body = CreatePlanRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Plan created", body = PlanDetailResponse),
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "day count mismatch"),
    )
)]
pub async fn create_plan(
    State(state): State<PlansState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Json(body): Json<CreatePlanRequest>,
) -> Result<(StatusCode, Json<PlanDetailResponse>), HttpError> {
    let input = CreatePlanInput {
        title: body.title,
        description: body.description,
        cover_url: body.cover_url,
        category: body.category,
        duration_days: body.duration_days,
        creator_id: *user.id().as_uuid(),
        days: body
            .days
            .into_iter()
            .map(|d| CreateDayInput {
                day_number: d.day_number,
                title: d.title,
                description: d.description,
                items: d
                    .items
                    .into_iter()
                    .map(|i| CreateItemInput {
                        book_id: i.book_id,
                        from_locator: i.from_locator,
                        to_locator: i.to_locator,
                        label: i.label,
                    })
                    .collect(),
            })
            .collect(),
    };

    let detail = CreatePlan {
        repository: state.repo,
    }
    .execute(input)
    .await
    .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(PlanDetailResponse::from(detail))))
}

#[utoipa::path(
    get,
    path = "/v1/plans/{id}",
    tag = "plans",
    params(("id" = Uuid, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan detail", body = PlanDetailResponse),
        (status = 404, description = "Not found"),
    )
)]
pub async fn get_plan(
    State(state): State<PlansState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PlanDetailResponse>, HttpError> {
    let detail = state
        .repo
        .get_plan(id)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError::from(AppError::not_found("plan")))?;

    Ok(Json(PlanDetailResponse::from(detail)))
}

// ── Subscriptions ─────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/v1/plans/{id}/subscribe",
    tag = "plans",
    params(("id" = Uuid, Path, description = "Plan ID")),
    request_body = SubscribeRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Subscribed", body = SubscriptionResponse),
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Already subscribed"),
    )
)]
pub async fn subscribe(
    State(state): State<PlansState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(plan_id): Path<Uuid>,
    Json(body): Json<SubscribeRequest>,
) -> Result<(StatusCode, Json<SubscriptionResponse>), HttpError> {
    let user_id = *user.id().as_uuid();

    // Conflict if already subscribed
    let existing = state
        .repo
        .get_subscription(user_id, plan_id)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    if existing.is_some() {
        return Err(HttpError::from(AppError::UnprocessableEntity(
            "already subscribed to this plan".into(),
        )));
    }

    let sub = state
        .repo
        .subscribe(user_id, plan_id, body.group_id)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(SubscriptionResponse::from(sub))))
}

#[utoipa::path(
    get,
    path = "/v1/plans/subscriptions",
    tag = "plans",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User subscriptions", body = Vec<SubscriptionResponse>),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn list_subscriptions(
    State(state): State<PlansState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
) -> Result<Json<Vec<SubscriptionResponse>>, HttpError> {
    let subs = state
        .repo
        .list_subscriptions(*user.id().as_uuid())
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok(Json(
        subs.into_iter().map(SubscriptionResponse::from).collect(),
    ))
}

// ── Progress ──────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/v1/plans/subscriptions/{sub_id}/progress",
    tag = "plans",
    params(("sub_id" = Uuid, Path, description = "Subscription ID")),
    request_body = MarkDayRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Day marked complete", body = ProgressResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Subscription or plan not found"),
    )
)]
pub async fn mark_day_complete(
    State(state): State<PlansState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(sub_id): Path<Uuid>,
    Json(body): Json<MarkDayRequest>,
) -> Result<(StatusCode, Json<ProgressResponse>), HttpError> {
    let progress = UpdateProgress {
        repository: state.repo,
    }
    .execute(*user.id().as_uuid(), sub_id, body.plan_day_id)
    .await
    .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(ProgressResponse::from(progress))))
}

#[utoipa::path(
    get,
    path = "/v1/plans/subscriptions/{sub_id}/progress",
    tag = "plans",
    params(("sub_id" = Uuid, Path, description = "Subscription ID")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Progress entries", body = Vec<ProgressResponse>),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn get_progress(
    State(state): State<PlansState>,
    JwtAuthExtractor(_user): JwtAuthExtractor,
    Path(sub_id): Path<Uuid>,
) -> Result<Json<Vec<ProgressResponse>>, HttpError> {
    let progress = state
        .repo
        .get_progress(sub_id)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok(Json(
        progress.into_iter().map(ProgressResponse::from).collect(),
    ))
}

// ── Groups ────────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/v1/plans/{id}/groups",
    tag = "plans",
    params(("id" = Uuid, Path, description = "Plan ID")),
    request_body = CreateGroupRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Group created", body = GroupResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Plan not found"),
    )
)]
pub async fn create_group(
    State(state): State<PlansState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Path(plan_id): Path<Uuid>,
    Json(body): Json<CreateGroupRequest>,
) -> Result<(StatusCode, Json<GroupResponse>), HttpError> {
    // Verify plan exists
    state
        .repo
        .get_plan(plan_id)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError::from(AppError::not_found("plan")))?;

    let group = state
        .repo
        .create_group(plan_id, *user.id().as_uuid(), body.name)
        .await
        .map_err(AppError::internal)
        .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(GroupResponse::from(group))))
}

#[utoipa::path(
    post,
    path = "/v1/plans/groups/join",
    tag = "plans",
    request_body = JoinGroupRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Joined group", body = GroupMemberResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Invite code not found"),
    )
)]
pub async fn join_group(
    State(state): State<PlansState>,
    JwtAuthExtractor(user): JwtAuthExtractor,
    Json(body): Json<JoinGroupRequest>,
) -> Result<(StatusCode, Json<GroupMemberResponse>), HttpError> {
    let member = JoinGroup {
        repository: state.repo,
    }
    .execute(&body.invite_code, *user.id().as_uuid())
    .await
    .map_err(HttpError::from)?;

    Ok((StatusCode::CREATED, Json(GroupMemberResponse::from(member))))
}
