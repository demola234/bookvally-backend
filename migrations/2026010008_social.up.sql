-- ============================================================
-- 2026010008_social.up.sql
-- friendships, invites, notifications
-- ============================================================

CREATE TABLE friendships (
    id           uuid              PRIMARY KEY DEFAULT gen_random_uuid(),
    requester_id uuid              NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    addressee_id uuid              NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    status       friendship_status NOT NULL DEFAULT 'pending',
    context      varchar(120),
    created_at   timestamptz       NOT NULL DEFAULT now(),
    responded_at timestamptz,
    CONSTRAINT uq_friendships_pair UNIQUE (requester_id, addressee_id)
);

CREATE INDEX idx_friendships_addressee ON friendships (addressee_id);
CREATE INDEX idx_friendships_status    ON friendships (status);

CREATE TABLE invites (
    id            uuid           PRIMARY KEY DEFAULT gen_random_uuid(),
    inviter_id    uuid           NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    code          varchar(40)    NOT NULL,
    channel       invite_channel NOT NULL,
    invited_email citext,
    accepted_by   uuid           REFERENCES users (id) ON DELETE SET NULL,
    status        varchar(20)    NOT NULL DEFAULT 'pending',
    created_at    timestamptz    NOT NULL DEFAULT now(),
    accepted_at   timestamptz,
    CONSTRAINT uq_invites_code UNIQUE (code)
);

CREATE INDEX idx_invites_inviter_id ON invites (inviter_id);

CREATE TABLE notifications (
    id         uuid              PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    uuid              NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    type       notification_type NOT NULL,
    actor_id   uuid              REFERENCES users (id) ON DELETE SET NULL,
    book_id    uuid              REFERENCES books (id) ON DELETE SET NULL,
    payload    jsonb,
    read_at    timestamptz,
    created_at timestamptz       NOT NULL DEFAULT now()
);

CREATE INDEX idx_notifications_user_read    ON notifications (user_id, read_at);
CREATE INDEX idx_notifications_user_created ON notifications (user_id, created_at);
