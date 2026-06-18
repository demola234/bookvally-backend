-- ============================================================
-- 2026010004_library_reading.up.sql
-- library_items, reading_sessions, highlights, bookmarks
-- NOTE: reading_sessions.voice_id FK added in 2026010005_tts
-- ============================================================

CREATE TABLE library_items (
    id              uuid           PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         uuid           NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    book_id         uuid           NOT NULL REFERENCES books (id),
    book_file_id    uuid           REFERENCES book_files (id) ON DELETE SET NULL,
    status          library_status NOT NULL DEFAULT 'queued',
    current_page    int            DEFAULT 0,
    current_locator text,
    progress_pct    numeric(5,2)   DEFAULT 0,
    rating          smallint,
    added_via       added_via      NOT NULL DEFAULT 'import',
    started_at      timestamptz,
    finished_at     timestamptz,
    last_read_at    timestamptz,
    created_at      timestamptz    NOT NULL DEFAULT now(),
    CONSTRAINT uq_library_items_user_book UNIQUE (user_id, book_id)
);

CREATE INDEX idx_library_items_user_status   ON library_items (user_id, status);
CREATE INDEX idx_library_items_user_last_read ON library_items (user_id, last_read_at);

-- voice_id is a bare column here; FK constraint added after voices table in 0005
CREATE TABLE reading_sessions (
    id              uuid         PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         uuid         NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    library_item_id uuid         NOT NULL REFERENCES library_items (id) ON DELETE CASCADE,
    mode            session_mode NOT NULL,
    voice_id        uuid,
    started_at      timestamptz  NOT NULL,
    ended_at        timestamptz,
    pages_read      int          DEFAULT 0,
    minutes         numeric(6,2) DEFAULT 0,
    created_at      timestamptz  NOT NULL DEFAULT now()
);

CREATE INDEX idx_reading_sessions_user_started ON reading_sessions (user_id, started_at);
CREATE INDEX idx_reading_sessions_library_item ON reading_sessions (library_item_id);

CREATE TABLE highlights (
    id              uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    library_item_id uuid        NOT NULL REFERENCES library_items (id) ON DELETE CASCADE,
    user_id         uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    color           text        NOT NULL,
    locator_start   text        NOT NULL,
    locator_end     text        NOT NULL,
    selected_text   text,
    note            text,
    created_at      timestamptz NOT NULL DEFAULT now(),
    updated_at      timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_highlights_library_item ON highlights (library_item_id);
CREATE INDEX idx_highlights_user_id      ON highlights (user_id);

CREATE TABLE bookmarks (
    id              uuid         PRIMARY KEY DEFAULT gen_random_uuid(),
    library_item_id uuid         NOT NULL REFERENCES library_items (id) ON DELETE CASCADE,
    locator         text         NOT NULL,
    page            int,
    label           varchar(140),
    created_at      timestamptz  NOT NULL DEFAULT now()
);

CREATE INDEX idx_bookmarks_library_item ON bookmarks (library_item_id);
