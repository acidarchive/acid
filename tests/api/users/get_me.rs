use crate::helpers::spawn_app;
use serde_json::json;

#[tokio::test]
async fn get_me_returns_401_without_auth() {
    let app = spawn_app().await;

    let response = app.get_user_me(None).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn get_me_returns_200_and_creates_user_on_first_call() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let user_id = app.get_test_user_id().await;

    let response = app.get_user_me(Some(token)).await;

    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["user_id"].as_str().unwrap(), user_id.to_string());
    assert!(body["avatar_url"].is_null());
    assert!(body["banner_url"].is_null());
    assert!(body["created_at"].is_string());
    assert!(body["updated_at"].is_string());
}

#[tokio::test]
async fn get_me_returns_same_user_on_subsequent_calls() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let response1 = app.get_user_me(Some(token.clone())).await;
    assert_eq!(response1.status().as_u16(), 200);
    let body1: serde_json::Value = response1.json().await.unwrap();

    let response2 = app.get_user_me(Some(token)).await;
    assert_eq!(response2.status().as_u16(), 200);
    let body2: serde_json::Value = response2.json().await.unwrap();

    assert_eq!(body1["user_id"], body2["user_id"]);
    assert_eq!(body1["created_at"], body2["created_at"]);
}

#[tokio::test]
async fn get_me_returns_urls_after_patch() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let user_id = app.get_test_user_id().await;

    let avatar_key = app.get_presign_key(&token, "avatar").await;
    let banner_key = app.get_presign_key(&token, "banner").await;

    let patch_body = json!({
        "avatar_key": avatar_key,
        "banner_key": banner_key
    });

    app.patch_user_me(patch_body.to_string(), Some(token.clone()))
        .await;

    let response = app.get_user_me(Some(token)).await;

    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    let avatar_url = body["avatar_url"].as_str().unwrap();
    let banner_url = body["banner_url"].as_str().unwrap();

    assert!(avatar_url.contains(&format!("avatars/{}/", user_id)));
    assert!(banner_url.contains(&format!("banners/{}/", user_id)));
}
