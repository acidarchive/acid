use crate::authentication::reject_unauthorized_users;
use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::{health_check, patterns};
use crate::utils::get_error_response;
use actix_cors::Cors;
use actix_web::{
    dev::Server, error, error::JsonPayloadError, middleware::from_fn, web, web::Data,
    web::JsonConfig, App, HttpResponse, HttpServer,
};
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

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
        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.cognito,
        )
        .await?;

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

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: Secret<String>,
    cognito_settings: crate::configuration::CognitoSettings,
) -> Result<Server, anyhow::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(patterns::create_tb303_pattern,),
        components(schemas(patterns::PatternTB303Request, patterns::PatternTB303Response,))
    )]
    struct ApiDoc;

    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
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
                web::scope("/api/v1").service(
                    web::scope("/patterns")
                        .route("/tb303", web::post().to(patterns::create_tb303_pattern))
                        .wrap(from_fn(reject_unauthorized_users)),
                ),
            )
            .service(Redoc::with_url("/docs/api", ApiDoc::openapi()))
            .route("/health_check", web::get().to(health_check))
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
            .app_data(cognito_settings.clone())
            .app_data(ApiError::json_error(JsonConfig::default()))
            .app_data(Data::new(HmacSecret(hmac_secret.expose_secret().clone())))
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
