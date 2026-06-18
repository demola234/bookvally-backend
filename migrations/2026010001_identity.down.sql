-- 2026010001_identity.down.sql
DROP TABLE IF EXISTS oauth_accounts;
DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS subscription_status;
DROP TYPE IF EXISTS league_result;
DROP TYPE IF EXISTS achievement_category;
DROP TYPE IF EXISTS xp_source;
DROP TYPE IF EXISTS freeze_source;
DROP TYPE IF EXISTS freeze_status;
DROP TYPE IF EXISTS streak_day_status;
DROP TYPE IF EXISTS device_platform;
DROP TYPE IF EXISTS profile_visibility;
DROP TYPE IF EXISTS account_status;
DROP TYPE IF EXISTS notification_type;
DROP TYPE IF EXISTS reader_theme;
DROP TYPE IF EXISTS app_theme;
DROP TYPE IF EXISTS goal_type;
DROP TYPE IF EXISTS invite_channel;
DROP TYPE IF EXISTS friendship_status;
DROP TYPE IF EXISTS voice_tier;
DROP TYPE IF EXISTS session_mode;
DROP TYPE IF EXISTS added_via;
DROP TYPE IF EXISTS library_status;
DROP TYPE IF EXISTS import_status;
DROP TYPE IF EXISTS book_format;
DROP TYPE IF EXISTS cloud_provider;
DROP TYPE IF EXISTS oauth_provider;

DROP EXTENSION IF EXISTS citext;
