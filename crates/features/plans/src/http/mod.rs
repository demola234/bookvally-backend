pub mod dto;
pub mod routes;

use utoipa::OpenApi;

use crate::http::dto::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::list_plans,
        routes::create_plan,
        routes::get_plan,
        routes::subscribe,
        routes::list_subscriptions,
        routes::mark_day_complete,
        routes::get_progress,
        routes::create_group,
        routes::join_group,
    ),
    components(schemas(
        PlanResponse,
        PlanDayResponse,
        PlanDayItemResponse,
        PlanDetailResponse,
        SubscriptionResponse,
        ProgressResponse,
        GroupResponse,
        GroupMemberResponse,
        CreatePlanRequest,
        CreateDayRequest,
        CreateDayItemRequest,
        SubscribeRequest,
        MarkDayRequest,
        CreateGroupRequest,
        JoinGroupRequest,
    ))
)]
pub struct PlansApiDoc;
