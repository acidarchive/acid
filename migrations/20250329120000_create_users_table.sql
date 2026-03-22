CREATE TABLE users (
    user_id UUID,
    username TEXT NOT NULL,
    avatar_key TEXT,
    banner_key TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id)
);
