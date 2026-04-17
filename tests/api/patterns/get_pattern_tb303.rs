use crate::helpers::spawn_app;
use crate::test_data::get_valid_tb303_pattern_data;
use uuid::Uuid;

#[tokio::test]
async fn get_pattern_tb303_returns_404_for_non_existent_pattern_without_auth() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_pattern_tb303(&Uuid::new_v4(), None).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn get_pattern_tb303_returns_200_for_public_pattern_without_auth() {
    // Arrange
    let app = spawn_app().await;
    let pattern_ids = app
        .create_test_patterns(&Uuid::new_v4(), 1, Some(true))
        .await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    // Act
    let response = app.get_pattern_tb303(pattern_id, None).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json["title"], "Pattern 1");
    assert_eq!(json["author"], "Author 1");
}

#[tokio::test]
async fn get_pattern_tb303_returns_404_for_private_pattern_without_auth() {
    let app = spawn_app().await;
    let pattern_ids = app
        .create_test_patterns(&Uuid::new_v4(), 1, Some(false))
        .await;
    let pattern_id = pattern_ids.first().expect("No patterns created");

    let response = app.get_pattern_tb303(pattern_id, None).await;

    assert_eq!(404, response.status().as_u16());
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
async fn get_pattern_tb303_returns_bars_and_steps() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let body = get_valid_tb303_pattern_data(Some(true));
    let post_response = app.post_patterns_tb303(body, Some(token.clone())).await;
    let post_json = post_response.json::<serde_json::Value>().await.unwrap();
    let pattern_id = post_json["data"]["id"].as_str().unwrap();
    let pattern_id = uuid::Uuid::parse_str(pattern_id).unwrap();

    // Act
    let response = app.get_pattern_tb303(&pattern_id, Some(token)).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    let bars = json["bars"].as_array().unwrap();
    assert_eq!(bars.len(), 1);

    let steps = bars[0]["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 16);
    assert_eq!(steps[0]["note"], "D");
    assert_eq!(steps[0]["time"], "note");
    assert_eq!(steps[5]["note"], "B");
    assert_eq!(steps[5]["accent"], true);
    assert_eq!(steps[5]["slide"], true);
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
