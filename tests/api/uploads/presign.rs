use crate::helpers::spawn_app;
use serde_json::json;

#[tokio::test]
async fn presign_returns_401_without_auth() {
    let app = spawn_app().await;

    let body = json!({
        "upload_type": "avatar",
        "content_type": "image/png",
        "content_length": 1024
    });

    let response = app.post_presign(body.to_string(), None).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn presign_returns_400_for_invalid_upload_type() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let body = json!({
        "upload_type": "invalid",
        "content_type": "image/png",
        "content_length": 1024
    });

    let response = app.post_presign(body.to_string(), Some(token)).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn presign_returns_400_for_invalid_content_type() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let body = json!({
        "upload_type": "avatar",
        "content_type": "text/plain",
        "content_length": 1024
    });

    let response = app.post_presign(body.to_string(), Some(token)).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn presign_returns_200_for_valid_avatar_request() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let user_id = app.get_test_user_id().await;

    let body = json!({
        "upload_type": "avatar",
        "content_type": "image/png",
        "content_length": 1024
    });

    let response = app.post_presign(body.to_string(), Some(token)).await;

    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    let upload_url = body["upload_url"].as_str().unwrap();
    let key = body["key"].as_str().unwrap();

    assert!(upload_url.contains(&format!("avatars/{}", user_id)));
    assert!(upload_url.contains(&app.s3.bucket));

    let parts: Vec<&str> = key.split('/').collect();

    assert_eq!(parts.len(), 3); // avatars / user_id / uuid
    assert_eq!(parts[0], "avatars");
    assert_eq!(parts[1], user_id.to_string());
    assert!(uuid::Uuid::parse_str(parts[2]).is_ok());
}

#[tokio::test]
async fn presign_returns_200_for_valid_banner_request() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;
    let user_id = app.get_test_user_id().await;

    let body = json!({
        "upload_type": "banner",
        "content_type": "image/jpeg",
        "content_length": 1024
    });

    let response = app.post_presign(body.to_string(), Some(token)).await;

    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    let upload_url = body["upload_url"].as_str().unwrap();
    let key = body["key"].as_str().unwrap();

    assert!(upload_url.contains(&format!("banners/{}", user_id)));
    assert!(upload_url.contains(&app.s3.bucket));

    let parts: Vec<&str> = key.split('/').collect();

    assert_eq!(parts.len(), 3); // banners / user_id / uuid
    assert_eq!(parts[0], "banners");
    assert_eq!(parts[1], user_id.to_string());
    assert!(uuid::Uuid::parse_str(parts[2]).is_ok());
}

#[tokio::test]
async fn presign_accepts_all_valid_image_types() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let valid_types = vec!["image/jpeg", "image/png", "image/webp", "image/gif"];

    for content_type in valid_types {
        let body = json!({
            "upload_type": "avatar",
            "content_type": content_type,
            "content_length": 1024
        });

        let response = app
            .post_presign(body.to_string(), Some(token.clone()))
            .await;

        assert_eq!(
            response.status().as_u16(),
            200,
            "Expected 200 for content_type: {}",
            content_type
        );
    }
}

#[tokio::test]
async fn presign_returns_400_for_zero_content_length() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let body = json!({
        "upload_type": "avatar",
        "content_type": "image/png",
        "content_length": 0
    });

    let response = app.post_presign(body.to_string(), Some(token)).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn presign_returns_400_for_exceeding_max_size() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let cases = vec![
        ("avatar", 3 * 1024 * 1024), // Avatar max is 2MB
        ("banner", 6 * 1024 * 1024), // Banner max is 5MB
    ];

    for (upload_type, content_length) in cases {
        let body = json!({
            "upload_type": upload_type,
            "content_type": "image/png",
            "content_length": content_length
        });

        let response = app
            .post_presign(body.to_string(), Some(token.clone()))
            .await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Expected 400 for {} with size {}",
            upload_type,
            content_length
        );
    }
}

#[tokio::test]
async fn presign_accepts_avatar_at_max_size() {
    let app = spawn_app().await;
    let token = app.get_test_user_token().await;

    let body = json!({
        "upload_type": "avatar",
        "content_type": "image/png",
        "content_length": 2 * 1024 * 1024 // Avatar max is 2MB
    });

    let response = app.post_presign(body.to_string(), Some(token)).await;

    assert_eq!(response.status().as_u16(), 200);
}
