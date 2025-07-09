use crate::helpers::spawn_app;
use uuid::Uuid;

#[tokio::test]
async fn get_pattern_tb303_returns_401_for_unauthorized_requests() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_pattern_tb303(&Uuid::new_v4(), None).await;

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn get_pattern_tb303_returns_404_for_non_existent_pattern() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    // Act
    let response = app.get_pattern_tb303(&Uuid::new_v4(), Some(token)).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn get_pattern_tb303_returns_200_for_existing_unowned_public_pattern() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let pattern_ids = app
        .create_test_patterns(&Uuid::new_v4(), 1, Some(true))
        .await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // Act
    let response = app.get_pattern_tb303(pattern_id, Some(token)).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["title"], "Pattern 1");
    assert_eq!(json["author"], "Author 1");
}

#[tokio::test]
async fn get_pattern_tb303_returns_404_for_existing_unowned_private_pattern() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let pattern_ids = app
        .create_test_patterns(&Uuid::new_v4(), 1, Some(false))
        .await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // Act
    let response = app.get_pattern_tb303(pattern_id, Some(token)).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn get_pattern_tb303_returns_200_for_existing_owned_public_pattern() {
    // Arrange
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;
    let token = app.get_test_user_token().await;

    let pattern_ids = app.create_test_patterns(&user_id, 1, Some(true)).await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // Act
    let response = app.get_pattern_tb303(pattern_id, Some(token)).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["title"], "Pattern 1");
    assert_eq!(json["author"], "Author 1");
}

#[tokio::test]
async fn get_pattern_tb303_returns_200_for_existing_owned_private_pattern() {
    // Arrange
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;
    let token = app.get_test_user_token().await;

    let pattern_ids = app.create_test_patterns(&user_id, 1, None).await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // Act
    let response = app.get_pattern_tb303(pattern_id, Some(token)).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["title"], "Pattern 1");
    assert_eq!(json["author"], "Author 1");
}
#[tokio::test]
async fn get_pattern_tb303_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let pattern_ids = app
        .create_test_patterns(&Uuid::new_v4(), 1, Some(false))
        .await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // destroy the database
    sqlx::query!("ALTER TABLE patterns_tb303 DROP column title")
        .execute(&app.db_pool)
        .await
        .unwrap();

    // Act
    let response = app.get_pattern_tb303(pattern_id, Some(token)).await;

    // assert
    assert_eq!(response.status().as_u16(), 500);
}
