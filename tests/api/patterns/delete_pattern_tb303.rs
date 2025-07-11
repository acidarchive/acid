use crate::helpers::spawn_app;
use uuid::Uuid;

#[tokio::test]
async fn delete_pattern_tb303_returns_401_for_unauthorized_requests() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.delete_pattern_tb303(&Uuid::new_v4(), None).await;

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn delete_pattern_tb303_returns_404_for_non_existent_pattern() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    // Act
    let response = app.delete_pattern_tb303(&Uuid::new_v4(), Some(token)).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn delete_pattern_tb303_returns_403_for_unowned_pattern() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let pattern_ids = app
        .create_test_patterns(&Uuid::new_v4(), 1, Some(true))
        .await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // Act
    let response = app.delete_pattern_tb303(pattern_id, Some(token)).await;

    // Assert
    assert_eq!(403, response.status().as_u16());
}

#[tokio::test]
async fn delete_pattern_tb303_returns_204_for_owned_pattern() {
    // Arrange
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;
    let token = app.get_test_user_token().await;

    let pattern_ids = app.create_test_patterns(&user_id, 1, Some(true)).await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // Act
    let response = app
        .delete_pattern_tb303(pattern_id, Some(token.clone()))
        .await;

    // Assert
    assert_eq!(204, response.status().as_u16());

    // Make sure the pattern is gone
    let response = app.get_pattern_tb303(pattern_id, Some(token)).await;
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn delete_pattern_tb303_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;
    let token = app.get_test_user_token().await;

    let pattern_ids = app.create_test_patterns(&user_id, 1, Some(true)).await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // destroy the database
    sqlx::query!("ALTER TABLE patterns_tb303 DROP column user_id")
        .execute(&app.db_pool)
        .await
        .unwrap();

    // Act
    let response = app.delete_pattern_tb303(pattern_id, Some(token)).await;

    // assert
    assert_eq!(response.status().as_u16(), 500);
}
