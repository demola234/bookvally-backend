-- ============================================================
-- 2026010009_analytics_read_models.up.sql
-- Denormalized read models / projections for the Stats feature.
-- Rebuilt by consuming reading.events; all rebuildable from source tables.
-- ============================================================

-- Weekly reading summary per user (leaderboard + stats dashboard)
CREATE TABLE weekly_reading_summaries (
    id           uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    week_start   date        NOT NULL,
    minutes      numeric(8,2) NOT NULL DEFAULT 0,
    pages        int          NOT NULL DEFAULT 0,
    sessions     int          NOT NULL DEFAULT 0,
    books_finished int        NOT NULL DEFAULT 0,
    xp_earned    int          NOT NULL DEFAULT 0,
    updated_at   timestamptz  NOT NULL DEFAULT now(),
    CONSTRAINT uq_weekly_reading_summaries_user_week UNIQUE (user_id, week_start)
);

CREATE INDEX idx_weekly_reading_summaries_user ON weekly_reading_summaries (user_id, week_start DESC);

-- Monthly reading summary per user (annual goal progress, profile share card)
CREATE TABLE monthly_reading_summaries (
    id             uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id        uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    year           smallint    NOT NULL,
    month          smallint    NOT NULL,
    minutes        numeric(8,2) NOT NULL DEFAULT 0,
    pages          int          NOT NULL DEFAULT 0,
    sessions       int          NOT NULL DEFAULT 0,
    books_finished int          NOT NULL DEFAULT 0,
    updated_at     timestamptz  NOT NULL DEFAULT now(),
    CONSTRAINT uq_monthly_reading_summaries_user_ym UNIQUE (user_id, year, month)
);

CREATE INDEX idx_monthly_reading_summaries_user ON monthly_reading_summaries (user_id, year DESC, month DESC);

-- Per-book reading pace projection (shown on book detail screen)
CREATE TABLE book_pace_projections (
    id                  uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    library_item_id     uuid        NOT NULL REFERENCES library_items (id) ON DELETE CASCADE,
    avg_pages_per_day   numeric(6,2),
    avg_minutes_per_day numeric(6,2),
    projected_finish_on date,
    computed_at         timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT uq_book_pace_projections_user_item UNIQUE (user_id, library_item_id)
);

-- Genre breakdown snapshot (Explore / profile taste section)
CREATE TABLE user_genre_stats (
    id           uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    genre        varchar(80) NOT NULL,
    books_read   int         NOT NULL DEFAULT 0,
    minutes      numeric(10,2) NOT NULL DEFAULT 0,
    updated_at   timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT uq_user_genre_stats_user_genre UNIQUE (user_id, genre)
);

CREATE INDEX idx_user_genre_stats_user ON user_genre_stats (user_id);
