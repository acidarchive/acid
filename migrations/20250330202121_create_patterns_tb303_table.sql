CREATE TABLE patterns_tb303(
    pattern_id uuid,
    user_id uuid NOT NULL,
    name TEXT NOT NULL,
    author TEXT,
    title TEXT,
    description TEXT,
    triplets BOOLEAN DEFAULT FALSE,
    tempo INTEGER,
    waveform TEXT,
    is_public BOOLEAN DEFAULT FALSE,
    tuning INTEGER DEFAULT 0,
    cut_off_freq INTEGER DEFAULT 0,
    resonance INTEGER DEFAULT 0,
    env_mod INTEGER DEFAULT 0,
    decay INTEGER DEFAULT 0,
    accent INTEGER DEFAULT 0,
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    created_at timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY (pattern_id)
);

CREATE INDEX idx_patterns_tb303_created ON patterns_tb303(created_at);
CREATE INDEX idx_patterns_tb303_name_gin ON patterns_tb303 USING gin(to_tsvector('english', name));
CREATE INDEX idx_patterns_tb303_author_gin ON patterns_tb303 USING gin(to_tsvector('english', author));
CREATE INDEX idx_patterns_tb303_title_gin ON patterns_tb303 USING gin(to_tsvector('english', title));
