-- ============================================================
-- 2026010002_profile_session.up.sql
-- user_profiles, devices, sessions, reminders,
-- user_settings, reading_goals, user_stats
-- NOTE: user_settings.default_voice_id FK added in 0005_tts
-- ============================================================

CREATE TABLE user_profiles (
    user_id         uuid               PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    bio             varchar(280),
    pronouns        varchar(40),
    location        varchar(120),
    banner_url      text,
    visibility      profile_visibility NOT NULL DEFAULT 'friends',
    favorite_genres varchar(200),
    reading_since   date,
    followers_count int                DEFAULT 0,
    friends_count   int                DEFAULT 0,
    created_at      timestamptz        NOT NULL DEFAULT now(),
    updated_at      timestamptz        NOT NULL DEFAULT now()
);

CREATE TABLE devices (
    id           uuid            PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      uuid            NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    platform     device_platform NOT NULL,
    device_name  varchar(120),
    push_token   text,
    app_version  varchar(20),
    last_seen_at timestamptz,
    created_at   timestamptz     NOT NULL DEFAULT now()
);

CREATE INDEX idx_devices_user_id ON devices (user_id);

CREATE TABLE sessions (
    id                 uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id            uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    device_id          uuid        REFERENCES devices (id) ON DELETE SET NULL,
    refresh_token_hash varchar(255) NOT NULL,
    ip_address         varchar(45),
    user_agent         text,
    created_at         timestamptz NOT NULL DEFAULT now(),
    last_active_at     timestamptz,
    expires_at         timestamptz NOT NULL,
    revoked_at         timestamptz
);

CREATE INDEX idx_sessions_user_id ON sessions (user_id);
CREATE INDEX idx_sessions_user_revoked ON sessions (user_id, revoked_at);

CREATE TABLE reminders (
    id           uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    time_local   time        NOT NULL,
    days_of_week smallint    NOT NULL DEFAULT 127,
    type         varchar(30) NOT NULL DEFAULT 'reading',
    enabled      boolean     NOT NULL DEFAULT true,
    created_at   timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_reminders_user_id ON reminders (user_id);

-- default_voice_id left as bare column; FK wired in 2026010005_tts.up.sql
CREATE TABLE user_settings (
    user_id                 uuid         PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    app_theme               app_theme    NOT NULL DEFAULT 'auto',
    reader_theme            reader_theme NOT NULL DEFAULT 'sepia',
    reader_font_family      varchar(60)  DEFAULT 'Source Serif 4',
    reader_font_size        smallint     DEFAULT 16,
    default_voice_id        uuid,
    default_speed           numeric(3,1) DEFAULT 1.0,
    default_pitch           numeric(3,1) DEFAULT 1.0,
    sleep_timer_minutes     smallint     DEFAULT 30,
    daily_goal_minutes      smallint     NOT NULL DEFAULT 15,
    activity_sharing        boolean      NOT NULL DEFAULT true,
    contact_matching_opt_in boolean      NOT NULL DEFAULT false,
    updated_at              timestamptz  NOT NULL DEFAULT now()
);

CREATE TABLE reading_goals (
    id           uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    type         goal_type   NOT NULL,
    target       int         NOT NULL,
    period_start date        NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT uq_reading_goals_user_type_period UNIQUE (user_id, type, period_start)
);

CREATE TABLE user_stats (
    user_id              uuid        PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    books_finished_total int         DEFAULT 0,
    highlights_total     int         DEFAULT 0,
    minutes_total        bigint      DEFAULT 0,
    pages_total          bigint      DEFAULT 0,
    updated_at           timestamptz NOT NULL DEFAULT now()
);
