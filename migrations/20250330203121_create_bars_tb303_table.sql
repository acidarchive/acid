CREATE TABLE bars_tb303 (
    bar_id uuid,
    pattern_id uuid NOT NULL REFERENCES patterns_tb303(pattern_id) ON DELETE CASCADE,
    number INTEGER NOT NULL,
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    created_at timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY (bar_id)
);
