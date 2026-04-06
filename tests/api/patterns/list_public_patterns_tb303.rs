use crate::helpers::spawn_app;

#[tokio::test]
async fn list_public_patterns_tb303_returns_200() {
    let app = spawn_app().await;

    let response = app.list_public_patterns_tb303(None, None, None).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_empty_array_when_no_patterns() {
    let app = spawn_app().await;

    let response = app.list_public_patterns_tb303(None, None, None).await;

    assert_eq!(200, response.status().as_u16());

    let json = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json["data"].as_array().unwrap().len(), 0);
    assert_eq!(json["total"], 0);
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_public_patterns() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 2, Some(true)).await;

    let response = app.list_public_patterns_tb303(None, None, None).await;
    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["data"].as_array().unwrap().len(), 2);
    assert_eq!(json["total"], 2);
}

#[tokio::test]
async fn list_public_patterns_tb303_excludes_private_patterns() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 2, Some(false)).await;

    let response = app.list_public_patterns_tb303(None, None, None).await;
    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["data"].as_array().unwrap().len(), 0);
    assert_eq!(json["total"], 0);
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_patterns_from_multiple_users() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;
    let other_user_id = uuid::Uuid::new_v4();

    app.create_test_patterns(&user_id, 1, Some(true)).await;
    app.create_test_patterns(&other_user_id, 2, Some(true))
        .await;

    let response = app.list_public_patterns_tb303(None, None, None).await;
    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["data"].as_array().unwrap().len(), 3);
    assert_eq!(json["total"], 3);
}

#[tokio::test]
async fn list_public_patterns_tb303_sorts_by_created_at_desc() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 3, Some(true)).await;

    let response = app.list_public_patterns_tb303(None, None, None).await;
    let json = response.json::<serde_json::Value>().await.unwrap();
    let records = json["data"].as_array().unwrap();

    assert_eq!(records[0]["title"], "Pattern 3");
    assert_eq!(records[1]["title"], "Pattern 2");
    assert_eq!(records[2]["title"], "Pattern 1");
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_correct_response_shape() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 1, Some(true)).await;

    let response = app.list_public_patterns_tb303(None, None, None).await;
    let json = response.json::<serde_json::Value>().await.unwrap();

    assert!(json.get("data").is_some());
    assert!(json.get("total").is_some());
    assert!(json.get("limit").is_some());
    assert!(json.get("offset").is_some());

    let record = &json["data"][0];
    assert!(record.get("pattern_id").is_some());
    assert_eq!(record["name"], "Pattern 1");
    assert_eq!(record["is_public"], true);
    assert!(record.get("created_at").is_some());
    assert!(record.get("updated_at").is_some());
}

#[tokio::test]
async fn list_public_patterns_tb303_respects_limit() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 5, Some(true)).await;

    let response = app.list_public_patterns_tb303(Some(2), None, None).await;
    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["data"].as_array().unwrap().len(), 2);
    assert_eq!(json["total"], 5);
    assert_eq!(json["limit"], 2);
    assert_eq!(json["offset"], 0);
}

#[tokio::test]
async fn list_public_patterns_tb303_respects_offset() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 3, Some(true)).await;

    let response = app
        .list_public_patterns_tb303(Some(10), Some(2), None)
        .await;
    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["data"].as_array().unwrap().len(), 1);
    assert_eq!(json["total"], 3);
    assert_eq!(json["offset"], 2);
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_empty_when_offset_beyond_total() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 3, Some(true)).await;

    let response = app
        .list_public_patterns_tb303(Some(10), Some(99), None)
        .await;
    let json = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(json["data"].as_array().unwrap().len(), 0);
    assert_eq!(json["total"], 3);
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_400_for_invalid_limit() {
    let app = spawn_app().await;

    let response = app.list_public_patterns_tb303(Some(0), None, None).await;
    assert_eq!(400, response.status().as_u16());

    let response = app.list_public_patterns_tb303(Some(101), None, None).await;
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_400_for_negative_offset() {
    let app = spawn_app().await;

    let response = app.list_public_patterns_tb303(None, Some(-1), None).await;
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn list_public_patterns_tb303_sorts_by_created_at_asc() {
    let app = spawn_app().await;
    let user_id = app.get_test_user_id().await;

    app.create_test_patterns(&user_id, 3, Some(true)).await;

    let response = app
        .list_public_patterns_tb303(None, None, Some("asc"))
        .await;
    let json = response.json::<serde_json::Value>().await.unwrap();
    let records = json["data"].as_array().unwrap();

    assert_eq!(records[0]["title"], "Pattern 1");
    assert_eq!(records[1]["title"], "Pattern 2");
    assert_eq!(records[2]["title"], "Pattern 3");
}

#[tokio::test]
async fn list_public_patterns_tb303_returns_400_for_invalid_order() {
    let app = spawn_app().await;

    let response = app
        .list_public_patterns_tb303(None, None, Some("random"))
        .await;
    assert_eq!(400, response.status().as_u16());
}
