use acid::configuration::{get_configuration, CognitoSettings, DatabaseSettings};
use acid::startup::{get_connection_pool, Application};
use acid::telemetry::{get_subscriber, init_subscriber};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use reqwest::Client;
use secrecy::Secret;
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

    let url = format!("https://cognito-idp.{cognito_region}.amazonaws.com/");

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
        return Err(format!("Failed to authenticate: {error_text}").into());
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
            .post(format!("{}/v1/patterns/tb303", &self.address))
            .header("Content-Type", "application/json");

        let request = if let Some(token) = token {
            request.header("Authorization", format!("Bearer {token}"))
        } else {
            request
        };

        request
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_patterns_tb303_random(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/v1/patterns/tb303/random", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_pattern_tb303(
        &self,
        pattern_id: &Uuid,
        token: Option<String>,
    ) -> reqwest::Response {
        let url = format!("{}/v1/patterns/tb303/{}", &self.address, pattern_id);

        let request = self.api_client.get(&url);

        let request = if let Some(token) = token {
            request.header("Authorization", format!("Bearer {token}"))
        } else {
            request
        };

        request.send().await.expect("Failed to execute request.")
    }

    pub async fn get_patterns_tb303(
        &self,
        token: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
        sort_column: Option<&str>,
        sort_direction: Option<&str>,
        search: Option<&str>,
        search_columns: Option<&str>,
        is_public: Option<bool>,
    ) -> reqwest::Response {
        let mut url = format!("{}/v1/patterns/tb303", &self.address);
        let mut query_params = vec![];

        if let Some(p) = page {
            query_params.push(format!("page={p}"));
        }
        if let Some(ps) = page_size {
            query_params.push(format!("page_size={ps}"));
        }
        if let Some(sc) = sort_column {
            query_params.push(format!("sort_column={sc}"));
        }
        if let Some(sd) = sort_direction {
            query_params.push(format!("sort_direction={sd}"));
        }
        if let Some(s) = search {
            query_params.push(format!("search={s}"));
        }
        if let Some(sc) = search_columns {
            query_params.push(format!("search_columns={sc}"));
        }
        if let Some(ip) = is_public {
            query_params.push(format!("is_public={ip}"));
        }

        if !query_params.is_empty() {
            url.push('?');
            url.push_str(&query_params.join("&"));
        }

        let request = self.api_client.get(&url);

        let request = if let Some(token) = token {
            request.header("Authorization", format!("Bearer {token}"))
        } else {
            request
        };

        request.send().await.expect("Failed to execute request.")
    }

    pub async fn get_test_user_token(&self) -> String {
        get_user_token(
            dotenvy::var("TEST_USER_USERNAME").unwrap().as_str(),
            dotenvy::var("TEST_USER_PASSWORD").unwrap().as_str(),
            &self.cognito.user_pool_client_id,
            &self.cognito.region,
        )
        .await
        .expect("Failed to get test user token")
    }

    pub async fn get_test_user_id(&self) -> Uuid {
        let user_id = dotenvy::var("TEST_USER_ID").unwrap();

        Uuid::parse_str(user_id.as_str()).expect("Failed to parse test user ID")
    }

    pub async fn create_test_patterns(
        &self,
        user_id: &Uuid,
        count: usize,
        is_public: Option<bool>,
    ) -> Vec<Uuid> {
        let mut pattern_ids = vec![];
        let public_status = is_public.unwrap_or(false);

        for i in 0..count {
            let pattern_id = Uuid::new_v4();
            sqlx::query!(
                r#"
                INSERT INTO patterns_tb303 (pattern_id, user_id, name, author, title, is_public, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#,
                pattern_id,
                user_id,
                format!("Pattern {}", i + 1),
                format!("Author {}", i + 1),
                format!("Pattern {}", i + 1),
                public_status,
                chrono::Utc::now(),
                chrono::Utc::now(),
            )
                .execute(&self.db_pool)
                .await
                .expect("Failed to create test pattern");

            pattern_ids.push(pattern_id);
        }

        pattern_ids
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
        address: format!("http://localhost:{application_port}"),
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

    sqlx::query!("TRUNCATE TABLE steps_tb303, patterns_tb303 RESTART IDENTITY CASCADE")
        .execute(&connection_pool)
        .await
        .expect("Failed to clean test database");

    connection_pool
}
