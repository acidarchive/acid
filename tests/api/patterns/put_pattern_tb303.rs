use crate::helpers::spawn_app;
use crate::test_data::get_valid_tb303_pattern_data;
use uuid::Uuid;

#[tokio::test]
async fn put_pattern_tb303_returns_401_for_unauthorized_requests() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_tb303_pattern_data(None);

    // Act
    let response = app.put_pattern_tb303(&Uuid::new_v4(), body, None).await;

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn put_pattern_tb303_returns_404_for_non_existent_pattern() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_tb303_pattern_data(None);
    let token = app.get_test_user_token().await;

    // Act
    let response = app
        .put_pattern_tb303(&Uuid::new_v4(), body, Some(token))
        .await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn put_pattern_tb303_updates_existing_pattern() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let user_id = app.get_test_user_id().await;

    let pattern_ids = app.create_test_patterns(&user_id, 2, Some(true)).await;
    let pattern_id = pattern_ids.first().expect("No patterns created");
    let body = get_valid_tb303_pattern_data(None);

    // Act
    let response = app.put_pattern_tb303(pattern_id, body, Some(token)).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json["data"]["id"], pattern_id.to_string());
}

#[tokio::test]
async fn put_pattern_tb303_returns_404_for_pattern_not_owned_by_user() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let other_user_id = Uuid::new_v4();
    let pattern_ids = app
        .create_test_patterns(&other_user_id, 1, Some(true))
        .await;
    let pattern_id = pattern_ids.first().expect("No patterns created");
    let body = get_valid_tb303_pattern_data(None);

    // Act
    let response = app.put_pattern_tb303(pattern_id, body, Some(token)).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn put_pattern_tb303_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let pattern_ids = app
        .create_test_patterns(&app.get_test_user_id().await, 1, Some(true))
        .await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    let body = r#"{
        "name": "Invalid Pattern",
        "steps": [
            {"number": 1, "time": "invalid_time", "note": "C"}
        ]
    }"#
    .to_string();

    // Act
    let response = app.put_pattern_tb303(pattern_id, body, Some(token)).await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn put_pattern_tb303_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let pattern_ids = app
        .create_test_patterns(&app.get_test_user_id().await, 1, Some(true))
        .await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // destroy the database
    sqlx::query!("DROP TABLE steps_tb303",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    let body = get_valid_tb303_pattern_data(None);

    // Act
    let response = app.put_pattern_tb303(pattern_id, body, Some(token)).await;

    // assert
    assert_eq!(response.status().as_u16(), 500);
}
