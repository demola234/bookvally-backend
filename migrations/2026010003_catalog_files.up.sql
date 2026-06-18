-- ============================================================
-- 2026010003_catalog_files.up.sql
-- books, cloud_connections, book_files
-- ============================================================

CREATE TABLE books (
    id               uuid         PRIMARY KEY DEFAULT gen_random_uuid(),
    title            varchar(500) NOT NULL,
    author           varchar(300),
    published_year   smallint,
    isbn             varchar(20),
    genre            varchar(80),
    synopsis         text,
    total_pages      int,
    cover_url        text,
    is_public_domain boolean      NOT NULL DEFAULT false,
    metadata_source  varchar(80),
    created_at       timestamptz  NOT NULL DEFAULT now()
);

CREATE INDEX idx_books_isbn ON books (isbn);
CREATE INDEX idx_books_title_author ON books (title, author);

CREATE TABLE cloud_connections (
    id               uuid           PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id          uuid           NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    provider         cloud_provider NOT NULL,
    account_email    citext,
    status           varchar(20)    NOT NULL DEFAULT 'connected',
    access_token_ref varchar(255),
    connected_at     timestamptz    NOT NULL DEFAULT now(),
    last_synced_at   timestamptz,
    CONSTRAINT uq_cloud_connections_user_provider UNIQUE (user_id, provider)
);

CREATE TABLE book_files (
    id                  uuid           PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             uuid           NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    book_id             uuid           REFERENCES books (id) ON DELETE SET NULL,
    cloud_connection_id uuid           REFERENCES cloud_connections (id) ON DELETE SET NULL,
    source              cloud_provider NOT NULL,
    file_name           varchar(500)   NOT NULL,
    format              book_format    NOT NULL,
    size_bytes          bigint,
    storage_key         text,
    import_status       import_status  NOT NULL DEFAULT 'pending',
    import_progress     smallint       DEFAULT 0,
    imported_at         timestamptz,
    created_at          timestamptz    NOT NULL DEFAULT now()
);

CREATE INDEX idx_book_files_user_id ON book_files (user_id);
CREATE INDEX idx_book_files_book_id ON book_files (book_id);
CREATE INDEX idx_book_files_user_status ON book_files (user_id, import_status);
