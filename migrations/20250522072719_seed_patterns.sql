INSERT INTO patterns_tb303 (
    pattern_id, user_id, name, author, title, description, triplets, tempo, waveform, tuning,
    cut_off_freq, resonance, env_mod, decay, accent, updated_at, created_at, is_public
) VALUES (
             'a8fcebd7-10cf-4bfb-a828-bbd27109e15e',
             '0b3fd33f-b34b-47e7-9a08-3ac597874cde',
             'Demo pattern 1',
             'Bam Bam',
             'Where''s your child?',
             'TUNING +4 semitones (C=E)',
             FALSE,
             118,
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

INSERT INTO steps_tb303 (
    step_id, pattern_id, number, note, transpose, "time", accent, slide, updated_at, created_at
) VALUES
      (gen_random_uuid(), 'a8fcebd7-10cf-4bfb-a828-bbd27109e15e', 1, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'a8fcebd7-10cf-4bfb-a828-bbd27109e15e', 2, 'B', 'up', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'a8fcebd7-10cf-4bfb-a828-bbd27109e15e', 3, 'F#', 'up', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'a8fcebd7-10cf-4bfb-a828-bbd27109e15e', 4, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'a8fcebd7-10cf-4bfb-a828-bbd27109e15e', 5, 'Chigh', 'up', 'note', FALSE, FALSE, NOW(), NOW());


INSERT INTO patterns_tb303 (
    pattern_id, user_id, name, author, title, description, triplets, tempo, waveform, tuning,
    cut_off_freq, resonance, env_mod, decay, accent, updated_at, created_at, is_public
) VALUES (
             '8d9af141-1464-4484-9d45-693affcafe5b',
             '0b3fd33f-b34b-47e7-9a08-3ac597874cde',
             'Demo pattern 2',
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

INSERT INTO steps_tb303 (
    step_id, pattern_id, number, note, transpose, "time", accent, slide, updated_at, created_at
) VALUES
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 1, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 2, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 3, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 4, null, null, 'tied', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 5, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 6, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 7, null, null, 'tied', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 8, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 9, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 10, null, null, 'tied', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 11, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 12, 'D', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 13, null, null, 'tied', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 14, 'B', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 15, 'F', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '8d9af141-1464-4484-9d45-693affcafe5b', 16, null, null, 'tied', FALSE, FALSE, NOW(), NOW());

INSERT INTO patterns_tb303 (
    pattern_id, user_id, name, author, title, description, triplets, tempo, waveform, tuning,
    cut_off_freq, resonance, env_mod, decay, accent, updated_at, created_at, is_public
) VALUES (
             '53d4bc60-9b42-40bf-9d27-4bb16d07cc33',
             '0b3fd33f-b34b-47e7-9a08-3ac597874cde',
             'Demo pattern 3',
             'The Prodigy',
             'Claustrophobic string',
             'TUNING +9 Semitones (C=A)',
             FALSE,
             null,
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

INSERT INTO steps_tb303 (
    step_id, pattern_id, number, note, transpose, "time", accent, slide, updated_at, created_at
) VALUES
      (gen_random_uuid(), '53d4bc60-9b42-40bf-9d27-4bb16d07cc33', 1, 'A', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '53d4bc60-9b42-40bf-9d27-4bb16d07cc33', 2, 'A', 'down', 'note', FALSE, TRUE, NOW(), NOW()),
      (gen_random_uuid(), '53d4bc60-9b42-40bf-9d27-4bb16d07cc33', 3, 'A', 'up', 'note', TRUE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '53d4bc60-9b42-40bf-9d27-4bb16d07cc33', 4, 'A', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '53d4bc60-9b42-40bf-9d27-4bb16d07cc33', 5, 'A', 'up', 'note', FALSE, TRUE, NOW(), NOW()),
      (gen_random_uuid(), '53d4bc60-9b42-40bf-9d27-4bb16d07cc33', 6, 'A', 'down', 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '53d4bc60-9b42-40bf-9d27-4bb16d07cc33', 7, 'A', null, 'note', TRUE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), '53d4bc60-9b42-40bf-9d27-4bb16d07cc33', 8, 'G#', null, 'note', TRUE, TRUE, NOW(), NOW());

INSERT INTO patterns_tb303 (
    pattern_id, user_id, name, author, title, description, triplets, tempo, waveform, tuning,
    cut_off_freq, resonance, env_mod, decay, accent, updated_at, created_at, is_public
) VALUES (
             'c922fdd1-9146-4a4b-9a18-e37ecd0c41da',
             '0b3fd33f-b34b-47e7-9a08-3ac597874cde',
             'Demo pattern 4',
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

INSERT INTO steps_tb303 (
    step_id, pattern_id, number, note, transpose, "time", accent, slide, updated_at, created_at
) VALUES
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 1, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 2, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 3, 'E', null, 'note', TRUE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 4, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 5, 'C#', 'up', 'note', TRUE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 6, 'C#', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 7, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 8, 'A', null, 'note', TRUE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 9, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 10, 'A', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 11, 'A', null, 'note', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 12, null, null, 'rest', FALSE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 13, 'A', null, 'note', TRUE, FALSE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 14, 'G', null, 'note', TRUE, TRUE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 15, 'G', null, 'note', TRUE, TRUE, NOW(), NOW()),
      (gen_random_uuid(), 'c922fdd1-9146-4a4b-9a18-e37ecd0c41da', 16, 'G', null, 'rest', FALSE, FALSE, NOW(), NOW());