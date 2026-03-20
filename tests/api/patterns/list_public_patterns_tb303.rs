use crate::helpers::spawn_app;

#[tokio::test]
async fn list_public_patterns_tb303_returns_200() {
    let app = spawn_app().await;

    let response = app.list_public_patterns_tb303().await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_empty_array_when_no_patterns() {
    let app = spawn_app().await;

    let response = app.list_public_patterns_tb303().await;

    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_public_patterns() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 2, Some(true)).await;

    let response = app.list_public_patterns_tb303().await;

    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    let records = json.as_array().unwrap();
    assert_eq!(records.len(), 2);
}

#[tokio::test]
async fn list_public_patterns_tb303_excludes_private_patterns() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 2, Some(false)).await;

    let response = app.list_public_patterns_tb303().await;

    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_patterns_from_multiple_users() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;
    let other_user_id = uuid::Uuid::new_v4();

    app.create_test_patterns(&user_id, 1, Some(true)).await;
    app.create_test_patterns(&other_user_id, 2, Some(true))
        .await;

    let response = app.list_public_patterns_tb303().await;

    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json.as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn list_public_patterns_tb303_sorts_by_created_at_desc() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 3, Some(true)).await;

    let response = app.list_public_patterns_tb303().await;

    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    let records = json.as_array().unwrap();

    assert_eq!(records[0]["title"], "Pattern 3");
    assert_eq!(records[1]["title"], "Pattern 2");
    assert_eq!(records[2]["title"], "Pattern 1");
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_correct_response_shape() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 1, Some(true)).await;

    let response = app.list_public_patterns_tb303().await;

    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    let record = &json[0];

    assert!(record.get("pattern_id").is_some());
    assert_eq!(record["name"], "Pattern 1");
    assert_eq!(record["is_public"], true);
    assert!(record.get("created_at").is_some());
    assert!(record.get("updated_at").is_some());
}
