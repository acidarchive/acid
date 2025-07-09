use crate::helpers::spawn_app;
use crate::test_data::get_valid_tb303_pattern_data;

#[tokio::test]
async fn get_patterns_tb303_random_returns_404_when_no_patterns_exist() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_patterns_tb303_random().await;

    // Assert
    assert_eq!(response.status().as_u16(), 404);
}

#[tokio::test]
async fn get_patterns_tb303_random_returns_404_when_no_public_patterns_exist() {
    // Arrange
    let app = spawn_app().await;

    let body = get_valid_tb303_pattern_data(None);
    let token = Some(app.get_test_user_token().await);
    let post_response = app.post_patterns_tb303(body, token).await;
    assert_eq!(post_response.status().as_u16(), 200);

    // Act
    let response = app.get_patterns_tb303_random().await;

    // Assert
    assert_eq!(response.status().as_u16(), 404);
}

#[tokio::test]
async fn get_patterns_tb303_random_returns_a_random_pattern() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_tb303_pattern_data(Some(true));
    let token = Some(app.get_test_user_token().await);

    let post_response = app.post_patterns_tb303(body, token).await;
    assert_eq!(post_response.status().as_u16(), 200);

    // Act
    let response = app.get_patterns_tb303_random().await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);

    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["title"], "Stakker humanoid");
    assert_eq!(json["author"], "Humanoind");
    assert_eq!(json["waveform"], "sawtooth");
    assert_eq!(json["tempo"], 130);
    assert!(json["steps"].is_array());
    assert_eq!(json["steps"].as_array().unwrap().len(), 16);
}
