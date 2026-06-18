-- ============================================================
-- 2026010001_identity.up.sql
-- Enums + core identity: users, oauth_accounts
-- ============================================================

CREATE EXTENSION IF NOT EXISTS citext;

-- ── enums ────────────────────────────────────────────────────
CREATE TYPE oauth_provider AS ENUM ('apple', 'google', 'github', 'microsoft');
CREATE TYPE cloud_provider  AS ENUM ('google_drive', 'dropbox', 'icloud', 'onedrive', 'device');
CREATE TYPE book_format     AS ENUM ('pdf', 'epub');
CREATE TYPE import_status   AS ENUM ('pending', 'importing', 'completed', 'failed');
CREATE TYPE library_status  AS ENUM ('queued', 'reading', 'finished', 'abandoned');
CREATE TYPE added_via       AS ENUM ('import', 'device_upload', 'discover', 'seed');
CREATE TYPE session_mode    AS ENUM ('read', 'listen');
CREATE TYPE voice_tier      AS ENUM ('standard', 'neural', 'pro');
CREATE TYPE friendship_status AS ENUM ('pending', 'accepted', 'declined', 'blocked');
CREATE TYPE invite_channel  AS ENUM ('link', 'email', 'qr', 'contacts');
CREATE TYPE goal_type       AS ENUM ('annual_books', 'weekly_pages');
CREATE TYPE app_theme       AS ENUM ('light', 'dark', 'auto');
CREATE TYPE reader_theme    AS ENUM ('light', 'sepia', 'dark');
CREATE TYPE notification_type AS ENUM (
    'friend_request', 'friend_accepted', 'friend_finished_book', 'friend_started_book',
    'streak_reminder', 'streak_milestone', 'league_promoted', 'league_demoted',
    'plan_day_due', 'achievement_unlocked', 'import_complete'
);
CREATE TYPE account_status       AS ENUM ('active', 'suspended', 'deactivated');
CREATE TYPE profile_visibility   AS ENUM ('public', 'friends', 'private');
CREATE TYPE device_platform      AS ENUM ('ios', 'android', 'web');
CREATE TYPE streak_day_status    AS ENUM ('none', 'earned', 'frozen', 'repaired');
CREATE TYPE freeze_status        AS ENUM ('available', 'used', 'expired');
CREATE TYPE freeze_source        AS ENUM ('earned', 'purchased', 'gifted');
CREATE TYPE xp_source            AS ENUM (
    'reading_minutes', 'finished_chapter', 'finished_book',
    'daily_goal_met', 'plan_day_completed', 'achievement', 'streak_milestone'
);
CREATE TYPE achievement_category AS ENUM ('streak', 'volume', 'social', 'exploration', 'plans', 'listening');
CREATE TYPE league_result        AS ENUM ('promoted', 'demoted', 'stayed');
CREATE TYPE subscription_status  AS ENUM ('active', 'completed', 'paused', 'abandoned');

-- ── users ────────────────────────────────────────────────────
CREATE TABLE users (
    id                uuid           PRIMARY KEY DEFAULT gen_random_uuid(),
    handle            varchar(20)    NOT NULL,
    display_name      varchar(120),
    email             citext         UNIQUE,
    avatar_url        text,
    avatar_hue        smallint,
    terms_version     varchar(20),
    terms_accepted_at timestamptz,
    status            account_status NOT NULL DEFAULT 'active',
    timezone          varchar(40)    NOT NULL DEFAULT 'UTC',
    last_active_at    timestamptz,
    created_at        timestamptz    NOT NULL DEFAULT now(),
    updated_at        timestamptz    NOT NULL DEFAULT now(),
    CONSTRAINT users_handle_unique UNIQUE (handle)
);

CREATE INDEX idx_users_email ON users (email);

-- ── oauth_accounts ───────────────────────────────────────────
CREATE TABLE oauth_accounts (
    id                  uuid           PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             uuid           NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    provider            oauth_provider NOT NULL,
    provider_account_id varchar(255)   NOT NULL,
    email               citext,
    created_at          timestamptz    NOT NULL DEFAULT now(),
    CONSTRAINT uq_oauth_provider_uid UNIQUE (provider, provider_account_id)
);

CREATE INDEX idx_oauth_accounts_user_id ON oauth_accounts (user_id);
