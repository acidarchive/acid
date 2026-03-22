use crate::helpers::spawn_app;
use serde_json::json;

#[tokio::test]
async fn patch_me_returns_401_without_auth() {
    let app = spawn_app().await;

    let body = json!({ "avatar_key": "avatars/some-user/some-uuid" });
    let response = app.patch_user_me(body.to_string(), None).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn patch_me_returns_400_with_empty_body() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let body = json!({});
    let response = app.patch_user_me(body.to_string(), Some(token)).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn patch_me_sets_avatar_url() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let user_id = app.get_test_user_id().await;

    let avatar_key = app.get_presign_key(&token, "avatar").await;
    let body = json!({
        "avatar_key": avatar_key
    });

    let response = app.patch_user_me(body.to_string(), Some(token)).await;
    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    let avatar_url = body["avatar_url"].as_str().unwrap();

    assert!(avatar_url.contains(&format!("avatars/{}/", user_id)));
}

#[tokio::test]
async fn patch_me_sets_banner_url() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let user_id = app.get_test_user_id().await;

    let banner_key = app.get_presign_key(&token, "banner").await;
    let body = json!({
        "banner_key": banner_key
    });

    let response = app.patch_user_me(body.to_string(), Some(token)).await;
    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    let banner_url = body["banner_url"].as_str().unwrap();

    assert!(banner_url.contains(&format!("banners/{}/", user_id)));
}

#[tokio::test]
async fn patch_me_sets_both_urls() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let user_id = app.get_test_user_id().await;

    let avatar_key = app.get_presign_key(&token, "avatar").await;
    let banner_key = app.get_presign_key(&token, "banner").await;
    let body = json!({ "avatar_key": avatar_key, "banner_key": banner_key });

    let response = app.patch_user_me(body.to_string(), Some(token)).await;
    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["avatar_url"]
        .as_str()
        .unwrap()
        .contains(&format!("avatars/{}/", user_id)));
    assert!(body["banner_url"]
        .as_str()
        .unwrap()
        .contains(&format!("banners/{}/", user_id)));
}

#[tokio::test]
async fn patch_me_partial_update_does_not_overwrite_other_fields() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let user_id = app.get_test_user_id().await;

    // Set avatar first
    let avatar_key = app.get_presign_key(&token, "avatar").await;
    app.patch_user_me(
        json!({ "avatar_key": avatar_key }).to_string(),
        Some(token.clone()),
    )
    .await;

    // Then set banner
    let banner_key = app.get_presign_key(&token, "banner").await;
    let response = app
        .patch_user_me(json!({ "banner_key": banner_key }).to_string(), Some(token))
        .await;
    assert_eq!(response.status().as_u16(), 200);

    // Check if both fields are present
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["avatar_url"]
        .as_str()
        .unwrap()
        .contains(&format!("avatars/{}/", user_id)));
    assert!(body["banner_url"]
        .as_str()
        .unwrap()
        .contains(&format!("banners/{}/", user_id)));
}
