-- ============================================================
-- 2026062801_book_text_chunks.up.sql
-- TTS-ready text chunks extracted from book files
-- ============================================================

CREATE TABLE IF NOT EXISTS book_text_chunks (
    id              uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    book_file_id    uuid        NOT NULL REFERENCES book_files(id) ON DELETE CASCADE,
    chapter         int         NOT NULL,
    sequence        int         NOT NULL,   -- order within chapter, 0-based
    text            text        NOT NULL,   -- cleaned, TTS-ready paragraph group
    char_count      int         NOT NULL,
    created_at      timestamptz NOT NULL DEFAULT now(),

    UNIQUE (book_file_id, chapter, sequence)
);

CREATE INDEX IF NOT EXISTS idx_book_text_chunks_file_chapter
    ON book_text_chunks (book_file_id, chapter, sequence);
