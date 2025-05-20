use acid::configuration::{get_configuration, CognitoSettings, DatabaseSettings};
use acid::startup::{get_connection_pool, Application};
use acid::telemetry::{get_subscriber, init_subscriber};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CognitoAuthRequest {
    auth_parameters: std::collections::HashMap<String, String>,
    auth_flow: String,
    client_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CognitoAuthResponse {
    authentication_result: Option<AuthenticationResult>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticationResult {
    pub id_token: String,
}

pub async fn get_user_token(
    username: &str,
    password: &str,
    client_id: &str,
    cognito_region: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut auth_params = std::collections::HashMap::new();
    auth_params.insert("USERNAME".to_string(), username.to_string());
    auth_params.insert("PASSWORD".to_string(), password.to_string());

    let body = CognitoAuthRequest {
        auth_flow: "USER_PASSWORD_AUTH".to_string(),
        auth_parameters: auth_params,
        client_id: client_id.to_string(),
    };

    let url = format!("https://cognito-idp.{}.amazonaws.com/", cognito_region);

    let client = Client::new();
    let response = client
        .post(&url)
        .header(
            "X-Amz-Target",
            "AWSCognitoIdentityProviderService.InitiateAuth",
        )
        .header("Content-Type", "application/x-amz-json-1.1")
        .json(&body)
        .send()
        .await
        .expect("Failed to send request to Cognito");

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Failed to authenticate: {}", error_text).into());
    }

    let auth_response: CognitoAuthResponse = response.json().await?;

    auth_response
        .authentication_result
        .ok_or_else(|| "No authentication result returned".into())
        .map(|result| result.id_token)
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub api_client: Client,
    pub cognito: CognitoSettings,
}

impl TestApp {
    pub async fn post_patterns_tb303(
        &self,
        body: String,
        token: Option<String>,
    ) -> reqwest::Response {
        let request = self
            .api_client
            .post(&format!("{}/v1/patterns/tb303", &self.address))
            .header("Content-Type", "application/json");

        let request = if let Some(token) = token {
            request.header("Authorization", format!("Bearer {}", token))
        } else {
            request
        };

        request
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_test_user_token(&self) -> String {
        get_user_token(
            &self.cognito.test_user.username,
            &self.cognito.test_user.password.expose_secret(),
            &self.cognito.user_pool_client_id,
            &self.cognito.region,
        )
        .await
        .expect("Failed to get test user token")
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    dotenv().ok();

    // randomize configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        db_pool: get_connection_pool(&configuration.database),
        api_client: client,
        cognito: configuration.cognito,
    };

    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create database
    let maintenance_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: Secret::new("password".to_string()),
        ..config.clone()
    };
    let mut connection = PgConnection::connect_with(&maintenance_settings.connect_options())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // migrate database
    let connection_pool = PgPool::connect_with(config.connect_options())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
