-- ============================================================
-- 2026010006_streaks_gamification.up.sql
-- daily_activity, streaks, streak_freezes,
-- leagues, league_cohorts, league_memberships,
-- user_game_state, xp_events,
-- achievements, user_achievements, friend_streaks
-- ============================================================

CREATE TABLE daily_activity (
    id             uuid              PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id        uuid              NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    activity_date  date              NOT NULL,
    minutes        numeric(6,2)      DEFAULT 0,
    pages          int               DEFAULT 0,
    xp_earned      int               DEFAULT 0,
    met_daily_goal boolean           NOT NULL DEFAULT false,
    streak_status  streak_day_status NOT NULL DEFAULT 'none',
    created_at     timestamptz       NOT NULL DEFAULT now(),
    updated_at     timestamptz       NOT NULL DEFAULT now(),
    CONSTRAINT uq_daily_activity_user_date UNIQUE (user_id, activity_date)
);

CREATE TABLE streaks (
    user_id          uuid        PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    current_length   int         NOT NULL DEFAULT 0,
    longest_length   int         NOT NULL DEFAULT 0,
    started_on       date,
    last_extended_on date,
    freezes_equipped smallint    NOT NULL DEFAULT 0,
    updated_at       timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE streak_freezes (
    id          uuid          PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     uuid          NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    status      freeze_status NOT NULL DEFAULT 'available',
    source      freeze_source NOT NULL,
    acquired_at timestamptz   NOT NULL DEFAULT now(),
    used_on     date,
    created_at  timestamptz   NOT NULL DEFAULT now()
);

CREATE INDEX idx_streak_freezes_user_status ON streak_freezes (user_id, status);

CREATE TABLE leagues (
    id            uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    name          varchar(40) NOT NULL,
    level         smallint    NOT NULL,
    promote_count smallint    DEFAULT 10,
    demote_count  smallint    DEFAULT 5,
    CONSTRAINT uq_leagues_level UNIQUE (level)
);

CREATE TABLE league_cohorts (
    id         uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    league_id  uuid        NOT NULL REFERENCES leagues (id),
    week_start date        NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_league_cohorts_league_week ON league_cohorts (league_id, week_start);

CREATE TABLE league_memberships (
    id        uuid          PRIMARY KEY DEFAULT gen_random_uuid(),
    cohort_id uuid          NOT NULL REFERENCES league_cohorts (id),
    user_id   uuid          NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    xp        int           NOT NULL DEFAULT 0,
    rank      smallint,
    result    league_result,
    joined_at timestamptz   NOT NULL DEFAULT now(),
    CONSTRAINT uq_league_memberships_cohort_user UNIQUE (cohort_id, user_id)
);

CREATE INDEX idx_league_memberships_user_id ON league_memberships (user_id);

CREATE TABLE user_game_state (
    user_id                      uuid        PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    xp_total                     bigint      NOT NULL DEFAULT 0,
    level                        int         NOT NULL DEFAULT 1,
    gems                         int         NOT NULL DEFAULT 0,
    current_league_membership_id uuid        REFERENCES league_memberships (id) ON DELETE SET NULL,
    updated_at                   timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE xp_events (
    id         uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    amount     int         NOT NULL,
    source     xp_source   NOT NULL,
    ref_type   varchar(40),
    ref_id     uuid,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_xp_events_user_created ON xp_events (user_id, created_at);

CREATE TABLE achievements (
    id          uuid                 PRIMARY KEY DEFAULT gen_random_uuid(),
    code        varchar(60)          NOT NULL,
    name        varchar(120)         NOT NULL,
    description varchar(280),
    icon        varchar(60),
    category    achievement_category NOT NULL,
    tier        smallint             DEFAULT 1,
    threshold   int,
    created_at  timestamptz          NOT NULL DEFAULT now(),
    CONSTRAINT uq_achievements_code UNIQUE (code)
);

CREATE TABLE user_achievements (
    id             uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id        uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    achievement_id uuid        NOT NULL REFERENCES achievements (id),
    progress       int         DEFAULT 0,
    unlocked_at    timestamptz,
    is_pinned      boolean     NOT NULL DEFAULT false,
    CONSTRAINT uq_user_achievements_user_achievement UNIQUE (user_id, achievement_id)
);

CREATE TABLE friend_streaks (
    id               uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_low_id      uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    user_high_id     uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    current_length   int         NOT NULL DEFAULT 0,
    longest_length   int         NOT NULL DEFAULT 0,
    last_extended_on date,
    created_at       timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT uq_friend_streaks_pair UNIQUE (user_low_id, user_high_id)
);
