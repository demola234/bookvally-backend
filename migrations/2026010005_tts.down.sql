-- 2026010005_tts.down.sql
ALTER TABLE reading_sessions DROP CONSTRAINT IF EXISTS fk_reading_sessions_voice;
ALTER TABLE user_settings    DROP CONSTRAINT IF EXISTS fk_user_settings_default_voice;
DROP TABLE IF EXISTS voices;
