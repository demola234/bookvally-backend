-- ============================================================
-- 2026010005_tts.up.sql
-- voices + deferred FKs on user_settings and reading_sessions
-- ============================================================

CREATE TABLE voices (
    id          uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    name        varchar(60) NOT NULL,
    locale      varchar(10) NOT NULL,
    tier        voice_tier  NOT NULL,
    descriptor  varchar(80),
    avatar_hue  smallint,
    preview_url text,
    is_active   boolean     NOT NULL DEFAULT true,
    created_at  timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_voices_locale_tier ON voices (locale, tier);

-- Wire FK that could not be declared in 0002 (voices didn't exist yet)
ALTER TABLE user_settings
    ADD CONSTRAINT fk_user_settings_default_voice
    FOREIGN KEY (default_voice_id) REFERENCES voices (id) ON DELETE SET NULL;

-- Wire FK that could not be declared in 0004 (voices didn't exist yet)
ALTER TABLE reading_sessions
    ADD CONSTRAINT fk_reading_sessions_voice
    FOREIGN KEY (voice_id) REFERENCES voices (id) ON DELETE SET NULL;
