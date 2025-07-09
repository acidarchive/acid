use crate::api::models::tb303::{TB303Pattern, TB303Step};
use crate::routes::patterns;
use utoipa::OpenApi;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        patterns::create_tb303_pattern,
        patterns::get_random_tb303_pattern,
        patterns::list_tb303_patterns,
        patterns::get_tb303_pattern,
    ),
    components(
        schemas(
            TB303Pattern,
            TB303Step
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
