-- ============================================================
-- 2026010007_reading_plans.up.sql
-- reading_plans, plan_days, plan_day_items,
-- plan_groups, plan_group_members,
-- plan_subscriptions, plan_progress
-- ============================================================

CREATE TABLE reading_plans (
    id            uuid         PRIMARY KEY DEFAULT gen_random_uuid(),
    title         varchar(200) NOT NULL,
    description   text,
    cover_url     text,
    category      varchar(80),
    duration_days int          NOT NULL,
    is_official   boolean      NOT NULL DEFAULT false,
    creator_id    uuid         REFERENCES users (id) ON DELETE SET NULL,
    created_at    timestamptz  NOT NULL DEFAULT now()
);

CREATE TABLE plan_days (
    id          uuid         PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_id     uuid         NOT NULL REFERENCES reading_plans (id) ON DELETE CASCADE,
    day_number  int          NOT NULL,
    title       varchar(200),
    description text,
    CONSTRAINT uq_plan_days_plan_day UNIQUE (plan_id, day_number)
);

CREATE TABLE plan_day_items (
    id           uuid         PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_day_id  uuid         NOT NULL REFERENCES plan_days (id) ON DELETE CASCADE,
    book_id      uuid         REFERENCES books (id) ON DELETE SET NULL,
    from_locator text,
    to_locator   text,
    label        varchar(200)
);

CREATE TABLE plan_groups (
    id          uuid         PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_id     uuid         NOT NULL REFERENCES reading_plans (id) ON DELETE CASCADE,
    owner_id    uuid         NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name        varchar(120),
    invite_code varchar(40)  UNIQUE,
    created_at  timestamptz  NOT NULL DEFAULT now()
);

CREATE TABLE plan_group_members (
    id        uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id  uuid        NOT NULL REFERENCES plan_groups (id) ON DELETE CASCADE,
    user_id   uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    joined_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT uq_plan_group_members_group_user UNIQUE (group_id, user_id)
);

CREATE TABLE plan_subscriptions (
    id          uuid                PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     uuid                NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    plan_id     uuid                NOT NULL REFERENCES reading_plans (id) ON DELETE CASCADE,
    group_id    uuid                REFERENCES plan_groups (id) ON DELETE SET NULL,
    started_on  date                NOT NULL,
    current_day int                 DEFAULT 1,
    status      subscription_status NOT NULL DEFAULT 'active',
    created_at  timestamptz         NOT NULL DEFAULT now(),
    CONSTRAINT uq_plan_subscriptions_user_plan UNIQUE (user_id, plan_id)
);

CREATE TABLE plan_progress (
    id              uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    subscription_id uuid        NOT NULL REFERENCES plan_subscriptions (id) ON DELETE CASCADE,
    plan_day_id     uuid        NOT NULL REFERENCES plan_days (id) ON DELETE CASCADE,
    completed_at    timestamptz,
    CONSTRAINT uq_plan_progress_sub_day UNIQUE (subscription_id, plan_day_id)
);
