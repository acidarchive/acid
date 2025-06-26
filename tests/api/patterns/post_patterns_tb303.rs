use crate::helpers::spawn_app;
use crate::test_data::get_valid_tb303_pattern_data;

#[tokio::test]
async fn post_pattern_tb303_returns_401_for_unauthorized_requests() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_tb303_pattern_data(None);

    // Act
    let response = app.post_patterns_tb303(body, None).await;

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn post_pattern_tb303_persists_the_new_pattern() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_tb303_pattern_data(Some(true));

    let token = Some(app.get_test_user_token().await);

    let response = app.post_patterns_tb303(body, token).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!(
        "SELECT name, author, title, description, waveform, triplets, tempo, tuning, cut_off_freq, resonance, \
        env_mod, decay, accent, is_public FROM patterns_tb303"
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved pattern");

    assert_eq!(saved.name, "Pattern 1".to_string());
    assert_eq!(saved.author, Some("Humanoind".to_string()));
    assert_eq!(saved.title, Some("Stakker humanoid".to_string()));
    assert_eq!(
        saved.description,
        Some(
            "This is a demo pattern for the TB-303. It's a classic acid house pattern.".to_string()
        )
    );
    assert_eq!(saved.waveform, Some("sawtooth".to_string()));
    assert_eq!(saved.triplets, Some(true));
    assert_eq!(saved.tempo, Some(130));
    assert_eq!(saved.cut_off_freq, Some(10));
    assert_eq!(saved.resonance, Some(20));
    assert_eq!(saved.env_mod, Some(30));
    assert_eq!(saved.decay, Some(40));
    assert_eq!(saved.accent, Some(50));
    assert_eq!(saved.tuning, Some(60));
    assert_eq!(saved.is_public, Some(true));

    let saved_steps = sqlx::query!(
        "SELECT pattern_id, number, note, transpose, time, accent, slide
         FROM steps_tb303
         WHERE pattern_id = (SELECT pattern_id FROM patterns_tb303 LIMIT 1)
         ORDER BY number"
    )
    .fetch_all(&app.db_pool)
    .await
    .expect("Failed to fetch saved steps");

    assert_eq!(saved_steps.len(), 16);

    let step1 = &saved_steps[0];
    assert_eq!(step1.number, 1);
    assert_eq!(step1.note, Some("D".to_string()));
    assert_eq!(step1.time, Some("note".to_string()));
    assert_eq!(step1.accent, Some(false));
    assert_eq!(step1.slide, Some(false));

    let step6 = &saved_steps[5];
    assert_eq!(step6.number, 6);
    assert_eq!(step6.note, Some("B".to_string()));
    assert_eq!(step6.transpose, Some("down".to_string()));
    assert_eq!(step6.time, Some("note".to_string()));
    assert_eq!(step6.accent, Some(true));
    assert_eq!(step6.slide, Some(true));

    let step7 = &saved_steps[6];
    assert_eq!(step7.number, 7);
    assert_eq!(step7.note, None);
    assert_eq!(step7.time, Some("tied".to_string()));
}

#[tokio::test]
async fn post_pattern_tb303_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let test_cases = vec![
        (
            r#"{
                "name": "Test Pattern",
                "steps": []
            }"#,
            "Empty steps array",
        ),
        (
            r#"{
                "name": "Test Pattern",
            }"#,
            "Missing steps array",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "tempo": -10,
                "steps": [{"number": 1, "time": "note", "note": "C"}]
            }"#,
            "Negative tempo",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "tempo": 1000,
                "steps": [{"number": 1, "time": "note", "note": "C"}]
            }"#,
            "Tempo too high",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "waveform": "invalid_waveform",
                "steps": [{"number": 1, "time": "note", "note": "C"}]
            }"#,
            "Invalid waveform",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [{"number": 0, "time": "note", "note": "C"}]
            }"#,
            "Step number zero",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [{"number": 17, "time": "note", "note": "C"}]
            }"#,
            "Step number too high",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [{"number": 1, "time": "note", "note": "H"}]
            }"#,
            "Invalid note",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [{"number": 1, "time": "note", "note": "C", "transpose": "way_up"}]
            }"#,
            "Invalid transpose",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [{"number": 1, "time": "invalid_time", "note": "C"}]
            }"#,
            "Invalid time",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [{"number": 1, "note": "C"}]
            }"#,
            "Missing time field",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "cut_off_freq": 361,
                "steps": [{"number": 1, "time": "note", "note": "C"}]
            }"#,
            "cut_off_freq out of range",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "resonance": -5,
                "steps": [{"number": 1, "time": "note", "note": "C"}]
            }"#,
            "resonance out of range",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [
                    {"number": 1, "time": "note", "note": "C"},
                    {"number": 1, "time": "note", "note": "D"}
                ]
            }"#,
            "Duplicate step numbers",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [
                    {"number": 1, "time": "note", "note": "C"},
                    {"number": 3, "time": "note", "note": "D"}
                ]
            }"#,
            "Missing step in sequence",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [
                    {"number": 1, "time": "rest", "note": "C"}
                ]
            }"#,
            "Rest step with note",
        ),
        (
            r#"{
                "name": "Test Pattern",
                "steps": [
                    {"number": 1, "time": "rest", "transpose": "up"}
                ]
            }"#,
            "Rest step with transpose",
        ),
    ];

    let token = Some(app.get_test_user_token().await);

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app
            .post_patterns_tb303(invalid_body.into(), token.clone())
            .await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "Failed to reject invalid data: {error_message}"
        );
    }
}

#[tokio::test]
async fn post_pattern_tb303_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;

    // destroy the database
    sqlx::query!("DROP TABLE steps_tb303",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    let body = get_valid_tb303_pattern_data(None);

    let token = Some(app.get_test_user_token().await);

    // Act
    let response = app.post_patterns_tb303(body, token).await;

    // assert
    assert_eq!(response.status().as_u16(), 500);
}
