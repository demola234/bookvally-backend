use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use persistence::PgPool;
use uuid::Uuid;

use crate::application::ports::PlansRepository;
use crate::domain::{
    PlanDay, PlanDayItem, PlanDetail, PlanGroup, PlanGroupMember, PlanProgress, PlanSubscription,
    ReadingPlan, SubscriptionStatus,
};

#[derive(Clone)]
pub struct PgPlansRepository {
    db: PgPool,
}

impl PgPlansRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[derive(sqlx::FromRow)]
struct PlanRow {
    id: Uuid,
    title: String,
    description: Option<String>,
    cover_url: Option<String>,
    category: Option<String>,
    duration_days: i32,
    is_official: bool,
    creator_id: Option<Uuid>,
    created_at: DateTime<Utc>,
}

impl From<PlanRow> for ReadingPlan {
    fn from(r: PlanRow) -> Self {
        Self {
            id: r.id,
            title: r.title,
            description: r.description,
            cover_url: r.cover_url,
            category: r.category,
            duration_days: r.duration_days,
            is_official: r.is_official,
            creator_id: r.creator_id,
            created_at: r.created_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct PlanDayRow {
    id: Uuid,
    plan_id: Uuid,
    day_number: i32,
    title: Option<String>,
    description: Option<String>,
}

impl From<PlanDayRow> for PlanDay {
    fn from(r: PlanDayRow) -> Self {
        Self {
            id: r.id,
            plan_id: r.plan_id,
            day_number: r.day_number,
            title: r.title,
            description: r.description,
        }
    }
}

#[derive(sqlx::FromRow)]
struct PlanDayItemRow {
    id: Uuid,
    plan_day_id: Uuid,
    book_id: Option<Uuid>,
    from_locator: Option<String>,
    to_locator: Option<String>,
    label: Option<String>,
}

impl From<PlanDayItemRow> for PlanDayItem {
    fn from(r: PlanDayItemRow) -> Self {
        Self {
            id: r.id,
            plan_day_id: r.plan_day_id,
            book_id: r.book_id,
            from_locator: r.from_locator,
            to_locator: r.to_locator,
            label: r.label,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SubscriptionRow {
    id: Uuid,
    user_id: Uuid,
    plan_id: Uuid,
    group_id: Option<Uuid>,
    started_on: NaiveDate,
    current_day: Option<i32>,
    status: String,
    created_at: DateTime<Utc>,
}

impl TryFrom<SubscriptionRow> for PlanSubscription {
    type Error = anyhow::Error;
    fn try_from(r: SubscriptionRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: r.id,
            user_id: r.user_id,
            plan_id: r.plan_id,
            group_id: r.group_id,
            started_on: r.started_on,
            current_day: r.current_day.unwrap_or(1),
            status: SubscriptionStatus::try_from(r.status)?,
            created_at: r.created_at,
        })
    }
}

#[derive(sqlx::FromRow)]
struct ProgressRow {
    id: Uuid,
    subscription_id: Uuid,
    plan_day_id: Uuid,
    completed_at: Option<DateTime<Utc>>,
}

impl From<ProgressRow> for PlanProgress {
    fn from(r: ProgressRow) -> Self {
        Self {
            id: r.id,
            subscription_id: r.subscription_id,
            plan_day_id: r.plan_day_id,
            completed_at: r.completed_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct GroupRow {
    id: Uuid,
    plan_id: Uuid,
    owner_id: Uuid,
    name: Option<String>,
    invite_code: Option<String>,
    created_at: DateTime<Utc>,
}

impl From<GroupRow> for PlanGroup {
    fn from(r: GroupRow) -> Self {
        Self {
            id: r.id,
            plan_id: r.plan_id,
            owner_id: r.owner_id,
            name: r.name,
            invite_code: r.invite_code,
            created_at: r.created_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct GroupMemberRow {
    id: Uuid,
    group_id: Uuid,
    user_id: Uuid,
    joined_at: DateTime<Utc>,
}

impl From<GroupMemberRow> for PlanGroupMember {
    fn from(r: GroupMemberRow) -> Self {
        Self {
            id: r.id,
            group_id: r.group_id,
            user_id: r.user_id,
            joined_at: r.joined_at,
        }
    }
}

#[async_trait]
impl PlansRepository for PgPlansRepository {
    async fn list_plans(
        &self,
        category: Option<String>,
        official_only: bool,
    ) -> anyhow::Result<Vec<ReadingPlan>> {
        let rows = sqlx::query_as!(
            PlanRow,
            r#"
            SELECT id, title, description, cover_url, category,
                   duration_days, is_official, creator_id, created_at
            FROM   reading_plans
            WHERE  ($1::text IS NULL OR category = $1)
              AND  (NOT $2 OR is_official = true)
            ORDER  BY created_at DESC
            "#,
            category,
            official_only,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(rows.into_iter().map(ReadingPlan::from).collect())
    }

    async fn get_plan(&self, id: Uuid) -> anyhow::Result<Option<PlanDetail>> {
        let Some(plan_row) = sqlx::query_as!(
            PlanRow,
            r#"
            SELECT id, title, description, cover_url, category,
                   duration_days, is_official, creator_id, created_at
            FROM   reading_plans
            WHERE  id = $1
            "#,
            id,
        )
        .fetch_optional(&self.db)
        .await?
        else {
            return Ok(None);
        };

        let day_rows = sqlx::query_as!(
            PlanDayRow,
            r#"
            SELECT id, plan_id, day_number, title, description
            FROM   plan_days
            WHERE  plan_id = $1
            ORDER  BY day_number
            "#,
            id,
        )
        .fetch_all(&self.db)
        .await?;

        let day_ids: Vec<Uuid> = day_rows.iter().map(|d| d.id).collect();

        let item_rows = sqlx::query_as!(
            PlanDayItemRow,
            r#"
            SELECT id, plan_day_id, book_id, from_locator, to_locator, label
            FROM   plan_day_items
            WHERE  plan_day_id = ANY($1)
            "#,
            &day_ids,
        )
        .fetch_all(&self.db)
        .await?;

        let mut items_by_day: HashMap<Uuid, Vec<PlanDayItem>> = HashMap::new();
        for item in item_rows {
            items_by_day
                .entry(item.plan_day_id)
                .or_default()
                .push(PlanDayItem::from(item));
        }

        let days = day_rows
            .into_iter()
            .map(|d| {
                let items = items_by_day.remove(&d.id).unwrap_or_default();
                (PlanDay::from(d), items)
            })
            .collect();

        Ok(Some(PlanDetail {
            plan: ReadingPlan::from(plan_row),
            days,
        }))
    }

    async fn create_plan(
        &self,
        plan: ReadingPlan,
        days: Vec<(PlanDay, Vec<PlanDayItem>)>,
    ) -> anyhow::Result<PlanDetail> {
        let mut tx = self.db.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO reading_plans
                (id, title, description, cover_url, category,
                 duration_days, is_official, creator_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            plan.id,
            plan.title,
            plan.description,
            plan.cover_url,
            plan.category,
            plan.duration_days,
            plan.is_official,
            plan.creator_id,
            plan.created_at,
        )
        .execute(&mut *tx)
        .await?;

        for (day, items) in &days {
            sqlx::query!(
                r#"
                INSERT INTO plan_days (id, plan_id, day_number, title, description)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                day.id,
                day.plan_id,
                day.day_number,
                day.title,
                day.description,
            )
            .execute(&mut *tx)
            .await?;

            for item in items {
                sqlx::query!(
                    r#"
                    INSERT INTO plan_day_items
                        (id, plan_day_id, book_id, from_locator, to_locator, label)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    "#,
                    item.id,
                    item.plan_day_id,
                    item.book_id,
                    item.from_locator,
                    item.to_locator,
                    item.label,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok(PlanDetail { plan, days })
    }

    async fn subscribe(
        &self,
        user_id: Uuid,
        plan_id: Uuid,
        group_id: Option<Uuid>,
    ) -> anyhow::Result<PlanSubscription> {
        let row = sqlx::query_as!(
            SubscriptionRow,
            r#"
            INSERT INTO plan_subscriptions
                (id, user_id, plan_id, group_id, started_on, current_day, status, created_at)
            VALUES ($1, $2, $3, $4, current_date, 1, 'active'::subscription_status, now())
            RETURNING
                id, user_id, plan_id, group_id,
                started_on, current_day,
                status::text AS "status!",
                created_at
            "#,
            Uuid::new_v4(),
            user_id,
            plan_id,
            group_id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(PlanSubscription::try_from(row)?)
    }

    async fn get_subscription(
        &self,
        user_id: Uuid,
        plan_id: Uuid,
    ) -> anyhow::Result<Option<PlanSubscription>> {
        let row = sqlx::query_as!(
            SubscriptionRow,
            r#"
            SELECT id, user_id, plan_id, group_id,
                   started_on, current_day,
                   status::text AS "status!",
                   created_at
            FROM   plan_subscriptions
            WHERE  user_id = $1 AND plan_id = $2
            "#,
            user_id,
            plan_id,
        )
        .fetch_optional(&self.db)
        .await?;

        row.map(PlanSubscription::try_from).transpose()
    }

    async fn get_subscription_by_id(
        &self,
        subscription_id: Uuid,
    ) -> anyhow::Result<Option<PlanSubscription>> {
        let row = sqlx::query_as!(
            SubscriptionRow,
            r#"
            SELECT id, user_id, plan_id, group_id,
                   started_on, current_day,
                   status::text AS "status!",
                   created_at
            FROM   plan_subscriptions
            WHERE  id = $1
            "#,
            subscription_id,
        )
        .fetch_optional(&self.db)
        .await?;

        row.map(PlanSubscription::try_from).transpose()
    }

    async fn list_subscriptions(&self, user_id: Uuid) -> anyhow::Result<Vec<PlanSubscription>> {
        let rows = sqlx::query_as!(
            SubscriptionRow,
            r#"
            SELECT id, user_id, plan_id, group_id,
                   started_on, current_day,
                   status::text AS "status!",
                   created_at
            FROM   plan_subscriptions
            WHERE  user_id = $1
            ORDER  BY created_at DESC
            "#,
            user_id,
        )
        .fetch_all(&self.db)
        .await?;

        rows.into_iter().map(PlanSubscription::try_from).collect()
    }

    async fn update_subscription_status(
        &self,
        subscription_id: Uuid,
        status: SubscriptionStatus,
    ) -> anyhow::Result<()> {
        sqlx::query("UPDATE plan_subscriptions SET status = $2::subscription_status WHERE id = $1")
            .bind(subscription_id)
            .bind(status.as_str())
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn advance_current_day(&self, subscription_id: Uuid, day: i32) -> anyhow::Result<()> {
        sqlx::query("UPDATE plan_subscriptions SET current_day = $2 WHERE id = $1")
            .bind(subscription_id)
            .bind(day)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn mark_day_complete(
        &self,
        subscription_id: Uuid,
        plan_day_id: Uuid,
    ) -> anyhow::Result<PlanProgress> {
        let row = sqlx::query_as!(
            ProgressRow,
            r#"
            INSERT INTO plan_progress (id, subscription_id, plan_day_id, completed_at)
            VALUES ($1, $2, $3, now())
            ON CONFLICT (subscription_id, plan_day_id)
            DO UPDATE SET completed_at = EXCLUDED.completed_at
            RETURNING id, subscription_id, plan_day_id, completed_at
            "#,
            Uuid::new_v4(),
            subscription_id,
            plan_day_id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(PlanProgress::from(row))
    }

    async fn get_progress(&self, subscription_id: Uuid) -> anyhow::Result<Vec<PlanProgress>> {
        let rows = sqlx::query_as!(
            ProgressRow,
            r#"
            SELECT id, subscription_id, plan_day_id, completed_at
            FROM   plan_progress
            WHERE  subscription_id = $1
            ORDER  BY completed_at ASC NULLS LAST
            "#,
            subscription_id,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(rows.into_iter().map(PlanProgress::from).collect())
    }

    async fn count_completed_days(&self, subscription_id: Uuid) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!" FROM plan_progress
               WHERE subscription_id = $1 AND completed_at IS NOT NULL"#,
            subscription_id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(count)
    }

    async fn create_group(
        &self,
        plan_id: Uuid,
        owner_id: Uuid,
        name: Option<String>,
    ) -> anyhow::Result<PlanGroup> {
        let invite_code = Uuid::new_v4().to_string().replace('-', "")[..8].to_string();

        let row = sqlx::query_as!(
            GroupRow,
            r#"
            INSERT INTO plan_groups (id, plan_id, owner_id, name, invite_code, created_at)
            VALUES ($1, $2, $3, $4, $5, now())
            RETURNING id, plan_id, owner_id, name, invite_code, created_at
            "#,
            Uuid::new_v4(),
            plan_id,
            owner_id,
            name,
            invite_code,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(PlanGroup::from(row))
    }

    async fn find_group_by_invite_code(
        &self,
        invite_code: &str,
    ) -> anyhow::Result<Option<PlanGroup>> {
        let row = sqlx::query_as!(
            GroupRow,
            r#"
            SELECT id, plan_id, owner_id, name, invite_code, created_at
            FROM   plan_groups
            WHERE  invite_code = $1
            "#,
            invite_code,
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(row.map(PlanGroup::from))
    }

    async fn join_group(&self, group_id: Uuid, user_id: Uuid) -> anyhow::Result<PlanGroupMember> {
        let row = sqlx::query_as!(
            GroupMemberRow,
            r#"
            INSERT INTO plan_group_members (id, group_id, user_id, joined_at)
            VALUES ($1, $2, $3, now())
            ON CONFLICT (group_id, user_id) DO UPDATE SET joined_at = plan_group_members.joined_at
            RETURNING id, group_id, user_id, joined_at
            "#,
            Uuid::new_v4(),
            group_id,
            user_id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(PlanGroupMember::from(row))
    }

    async fn list_group_members(&self, group_id: Uuid) -> anyhow::Result<Vec<PlanGroupMember>> {
        let rows = sqlx::query_as!(
            GroupMemberRow,
            r#"
            SELECT id, group_id, user_id, joined_at
            FROM   plan_group_members
            WHERE  group_id = $1
            ORDER  BY joined_at ASC
            "#,
            group_id,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(rows.into_iter().map(PlanGroupMember::from).collect())
    }
}
