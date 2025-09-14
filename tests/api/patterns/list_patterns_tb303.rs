use crate::helpers::spawn_app;
use crate::test_data::get_valid_tb303_pattern_data;
use uuid::Uuid;

#[tokio::test]
async fn list_patterns_tb303_returns_401_for_unauthorized_requests() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.list_patterns_tb303(None).await;

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn list_patterns_tb303_returns_an_empty_array_when_no_patterns_exist() {
    // Arrange
    let app = spawn_app().await;

    let token = Some(app.get_test_user_token().await);

    // Act
    let response = app.list_patterns_tb303(token).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn list_patterns_tb303_returns_pattern_summary_correctly() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let body = get_valid_tb303_pattern_data(Some(true));
    app.post_patterns_tb303(body, Some(token.clone())).await;

    let response = app.list_patterns_tb303(Some(token)).await;

    assert_eq!(200, response.status().as_u16());

    // Assert
    let json = response.json::<serde_json::Value>().await.unwrap();
    let record = &json[0];

    assert_eq!(record["name"], "Pattern 1");
    assert_eq!(record["author"], "Humanoind");
    assert_eq!(record["title"], "Stakker humanoid");
    assert_eq!(record["is_public"], true);
    assert!(record.get("created_at").is_some());
    assert!(record.get("updated_at").is_some());
}

#[tokio::test]
async fn list_patterns_tb303_sorts_by_created_at_desc_by_default() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let user_id = app.get_test_user_id().await;
    app.create_test_patterns(&user_id, 3, None).await;

    // Act
    let response = app.list_patterns_tb303(Some(token)).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    let records = json.as_array().unwrap();

    assert_eq!(records[0]["title"], "Pattern 3");
    assert_eq!(records[1]["title"], "Pattern 2");
    assert_eq!(records[2]["title"], "Pattern 1");
}

#[tokio::test]
async fn get_patterns_tb303_only_returns_user_owned_patterns() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let user_id = app.get_test_user_id().await;
    let other_user_id: Uuid = Uuid::new_v4();

    app.create_test_patterns(&user_id, 3, None).await;
    app.create_test_patterns(&other_user_id, 2, None).await;

    // Act
    let response = app.list_patterns_tb303(Some(token)).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    let records = json.as_array().unwrap();

    assert_eq!(records.len(), 3);
}

#[tokio::test]
async fn list_patterns_tb303_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    // destroy the database
    sqlx::query!("ALTER TABLE patterns_tb303 DROP column title")
        .execute(&app.db_pool)
        .await
        .unwrap();

    // Act
    let response = app.list_patterns_tb303(Some(token)).await;

    // assert
    assert_eq!(response.status().as_u16(), 500);
}
