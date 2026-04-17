INSERT INTO users (user_id, username) VALUES ('26f29224-6001-702f-25dc-6d5c1b750f51', 'acid');

INSERT INTO patterns_tb303 (
    pattern_id, user_id, name, author, title, description, triplets, tempo, waveform, tuning,
    cut_off_freq, resonance, env_mod, decay, accent, updated_at, created_at, is_public
) VALUES (
    'a8fcebd7-10cf-4bfb-a828-bbd27109e15e',
    '26f29224-6001-702f-25dc-6d5c1b750f51',
    'Bam Bam - Where''s your child?',
    'Bam Bam',
    'Where''s your child?',
    null,
    FALSE,
    118,
    'square',
    70,
    50,
    50,
    50,
    50,
    50,
    NOW(),
    NOW(),
    TRUE
);

INSERT INTO bars_tb303 (
    bar_id, pattern_id, number, updated_at, created_at
) VALUES (
    'cf016b29-3690-4531-9962-f3bdc07d483e',
    'a8fcebd7-10cf-4bfb-a828-bbd27109e15e',
    1,
    NOW(),
    NOW()
);

INSERT INTO steps_tb303 (
    step_id, bar_id, number, note, transpose, "time", accent, slide, updated_at, created_at
) VALUES
    (gen_random_uuid(), 'cf016b29-3690-4531-9962-f3bdc07d483e', 1, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'cf016b29-3690-4531-9962-f3bdc07d483e', 2, 'B', 'up', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'cf016b29-3690-4531-9962-f3bdc07d483e', 3, 'F#', 'up', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'cf016b29-3690-4531-9962-f3bdc07d483e', 4, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'cf016b29-3690-4531-9962-f3bdc07d483e', 5, 'Chigh', 'up', 'note', FALSE, FALSE, NOW(), NOW()
);


INSERT INTO patterns_tb303 (
    pattern_id, user_id, name, author, title, description, triplets, tempo, waveform, tuning,
    cut_off_freq, resonance, env_mod, decay, accent, updated_at, created_at, is_public
) VALUES (
    '8d9af141-1464-4484-9d45-693affcafe5b',
    '26f29224-6001-702f-25dc-6d5c1b750f51',
    'Humanoid - Stakker humanoid',
    'Humanoid',
    'Stakker humanoid',
    null,
    FALSE,
    120,
    'square',
    50,
    50,
    50,
    50,
    50,
    50,
    NOW(),
    NOW(),
    TRUE
);

INSERT INTO bars_tb303 (
    bar_id, pattern_id, number, updated_at, created_at
) VALUES (
    '43b771c2-51ab-4d99-a693-4e52df3d5463',
    '8d9af141-1464-4484-9d45-693affcafe5b',
    1,
    NOW(),
    NOW()
);

INSERT INTO steps_tb303 (
    step_id, bar_id, number, note, transpose, "time", accent, slide, updated_at, created_at
) VALUES
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 1, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 2, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 3, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 4, null, null, 'tied', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 5, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 6, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 7, null, null, 'tied', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 8, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 9, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 10, null, null, 'tied', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 11, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 12, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 13, null, null, 'tied', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 14, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 15, 'F', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '43b771c2-51ab-4d99-a693-4e52df3d5463', 16, null, null, 'tied', FALSE, FALSE, NOW(), NOW()
);

INSERT INTO patterns_tb303 (
    pattern_id, user_id, name, author, title, description, triplets, tempo, waveform, tuning,
    cut_off_freq, resonance, env_mod, decay, accent, updated_at, created_at, is_public
) VALUES (
    '53d4bc60-9b42-40bf-9d27-4bb16d07cc33',
    '26f29224-6001-702f-25dc-6d5c1b750f51',
    'The Prodigy - Claustrophobic String',
    'The Prodigy',
    'Claustrophobic String',
    null,
    FALSE,
    null,
    'square',
    100,
    50,
    50,
    50,
    50,
    50,
    NOW(),
    NOW(),
    TRUE
);

INSERT INTO bars_tb303 (
    bar_id, pattern_id, number, updated_at, created_at
) VALUES (
    'c4a6be99-a757-41f1-a4c7-5aca7e3d4914',
    '53d4bc60-9b42-40bf-9d27-4bb16d07cc33',
    1,
    NOW(),
    NOW()
);

INSERT INTO steps_tb303 (
    step_id, bar_id, number, note, transpose, "time", accent, slide, updated_at, created_at
) VALUES
    (gen_random_uuid(), 'c4a6be99-a757-41f1-a4c7-5aca7e3d4914', 1, 'A', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c4a6be99-a757-41f1-a4c7-5aca7e3d4914', 2, 'A', 'down', 'note', FALSE, TRUE, NOW(), NOW()),
    (gen_random_uuid(), 'c4a6be99-a757-41f1-a4c7-5aca7e3d4914', 3, 'A', 'up', 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c4a6be99-a757-41f1-a4c7-5aca7e3d4914', 4, 'A', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c4a6be99-a757-41f1-a4c7-5aca7e3d4914', 5, 'A', 'up', 'note', FALSE, TRUE, NOW(), NOW()),
    (gen_random_uuid(), 'c4a6be99-a757-41f1-a4c7-5aca7e3d4914', 6, 'A', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c4a6be99-a757-41f1-a4c7-5aca7e3d4914', 7, 'A', null, 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c4a6be99-a757-41f1-a4c7-5aca7e3d4914', 8, 'G#', null, 'note', TRUE, TRUE, NOW(), NOW()
);

INSERT INTO patterns_tb303 (
    pattern_id, user_id, name, author, title, description, triplets, tempo, waveform, tuning,
    cut_off_freq, resonance, env_mod, decay, accent, updated_at, created_at, is_public
) VALUES (
    'c922fdd1-9146-4a4b-9a18-e37ecd0c41da',
    '26f29224-6001-702f-25dc-6d5c1b750f51',
    'A Guy Called Gerald - Voodoo Ray',
    'A Guy Called Gerald',
    'Voodoo Ray',
    null,
    FALSE,
    null,
    'sawtooth',
    50,
    50,
    50,
    50,
    50,
    50,
    NOW(),
    NOW(),
    TRUE
);

INSERT INTO bars_tb303 (
    bar_id, pattern_id, number, updated_at, created_at
) VALUES (
    'ffb40def-9ea9-4218-aabb-b1ebd23fab07',
    'c922fdd1-9146-4a4b-9a18-e37ecd0c41da',
    1,
    NOW(),
    NOW()
);


INSERT INTO steps_tb303 (
    step_id, bar_id, number, note, transpose, "time", accent, slide, updated_at, created_at
) VALUES
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 1, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 2, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 3, 'E', null, 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 4, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 5, 'C#', 'up', 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 6, 'C#', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 7, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 8, 'A', null, 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 9, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 10, 'A', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 11, 'A', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 12, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 13, 'A', null, 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 14, 'G', null, 'note', TRUE, TRUE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 15, 'G', null, 'note', TRUE, TRUE, NOW(), NOW()),
    (gen_random_uuid(), 'ffb40def-9ea9-4218-aabb-b1ebd23fab07', 16, 'G', null, 'rest', FALSE, FALSE, NOW(), NOW()
);

INSERT INTO patterns_tb303 (
    pattern_id, user_id, name, author, title, description, triplets, tempo, waveform, tuning,
    cut_off_freq, resonance, env_mod, decay, accent, updated_at, created_at, is_public
) VALUES (
    'eec1e68c-3d65-4ab5-8050-f77e0d186aef',
    '26f29224-6001-702f-25dc-6d5c1b750f51',
    'Public Energy - Three ''O Three',
    'Public Energy',
    'Three ''O Three',
    null,
    FALSE,
    null,
    'sawtooth',
    50,
    50,
    50,
    50,
    50,
    50,
    NOW(),
    NOW(),
    TRUE
);

INSERT INTO bars_tb303 (
    bar_id, pattern_id, number, updated_at, created_at
) VALUES
    ('c087264e-53e5-4faa-981b-0f61176264a6', 'eec1e68c-3d65-4ab5-8050-f77e0d186aef', 1, NOW(), NOW()),
    ('8759128b-9d14-433c-bf2f-d054ae612af7', 'eec1e68c-3d65-4ab5-8050-f77e0d186aef', 2, NOW(), NOW());


INSERT INTO steps_tb303 (
    step_id, bar_id, number, note, transpose, "time", accent, slide, updated_at, created_at
) VALUES
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 1, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 2, 'C#', null, 'note', FALSE, TRUE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 3, 'C#', null, 'note', FALSE, TRUE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 4, 'C#', null, 'note', FALSE, TRUE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 5, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 6, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 7, 'C#', 'down', 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 8, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 9, 'C#', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 10, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 11, 'C#', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 12, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 13, 'C#', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 14, 'F#', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 15, 'G#', null, 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), 'c087264e-53e5-4faa-981b-0f61176264a6', 16, 'B', 'up', 'note', FALSE, FALSE, NOW(), NOW()),

    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 1, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 2, 'B', 'down', 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 3, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 4, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 5, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 6, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 7, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 8, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 9, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 10, 'Chigh', 'up', 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 11, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 12, 'Chigh', 'up', 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 13, 'C', null, 'note', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 14, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 15, 'Chigh', 'up', 'note', TRUE, FALSE, NOW(), NOW()),
    (gen_random_uuid(), '8759128b-9d14-433c-bf2f-d054ae612af7', 16, null, null, 'rest', FALSE, FALSE, NOW(), NOW()
);
