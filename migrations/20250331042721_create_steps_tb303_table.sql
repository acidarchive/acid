CREATE TABLE steps_tb303(
    step_id uuid,
    bar_id uuid NOT NULL REFERENCES bars_tb303(bar_id) ON DELETE CASCADE,
    number INTEGER NOT NULL,
    note TEXT,
    transpose TEXT,
    "time" TEXT,
    accent BOOLEAN DEFAULT FALSE,
    slide BOOLEAN DEFAULT FALSE,
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    created_at timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY (step_id)
);
