CREATE TABLE patterns_tb303(
    pattern_id uuid,
    user_id uuid NOT NULL,
    author TEXT,
    title TEXT,
    description TEXT,
    triplets BOOLEAN DEFAULT FALSE,
    bpm INTEGER,
    waveform TEXT,
    cut_off_freq INTEGER DEFAULT 0,
    resonance INTEGER DEFAULT 0,
    env_mod INTEGER DEFAULT 0,
    decay INTEGER DEFAULT 0,
    accent INTEGER DEFAULT 0,
    updated_at timestamptz,
    created_at timestamptz,
    PRIMARY KEY (pattern_id)
);
