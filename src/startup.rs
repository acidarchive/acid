use crate::api_docs::ApiDoc;
use crate::authentication::reject_unauthorized_users;
use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::{health_check, patterns};
use crate::utils::get_error_response;
use actix_cors::Cors;
use actix_web::{
    dev::Server, error, error::JsonPayloadError, middleware::from_fn, web, web::Data,
    web::JsonConfig, App, HttpResponse, HttpServer,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener, connection_pool, configuration.cognito).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    cognito_settings: crate::configuration::CognitoSettings,
) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let cognito_settings = Data::new(cognito_settings);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .service(
                web::scope("/v1").service(
                    web::scope("/patterns")
                        .route(
                            "/tb303/random",
                            web::get().to(patterns::get_random_tb303_pattern),
                        )
                        .service(
                            web::scope("")
                                .wrap(from_fn(reject_unauthorized_users))
                                .route("/tb303", web::post().to(patterns::create_tb303_pattern)),
                        ),
                ),
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/docs"))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/health_check", web::get().to(health_check))
            .app_data(db_pool.clone())
            .app_data(cognito_settings.clone())
            .app_data(ApiError::json_error(JsonConfig::default()))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub String);

pub struct ApiError;

impl ApiError {
    pub fn json_error(cfg: JsonConfig) -> JsonConfig {
        cfg.limit(4096)
            .error_handler(|err: JsonPayloadError, _req| {
                let error = err.to_string();
                let slice = &error[..error.find(" at").unwrap()];

                // create custom error response
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(get_error_response(slice.to_string())),
                )
                .into()
            })
    }
}
