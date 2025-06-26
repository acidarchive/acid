use crate::configuration::CognitoSettings;
use actix_web::http::Method;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::InternalError,
    http::header::{HeaderMap, AUTHORIZATION},
    middleware::Next,
    web, HttpMessage, HttpResponse,
};
use anyhow::anyhow;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use once_cell::sync::Lazy;
use reqwest;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref, sync::RwLock};
use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CognitoClaims {
    sub: String,
    aud: String,
    iss: String,
    exp: usize,
    iat: usize,

    token_use: String,
}

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

static JWKS_CACHE: Lazy<RwLock<HashMap<String, DecodingKey>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

fn extract_token_from_header(headers: &HeaderMap) -> Result<&str, anyhow::Error> {
    let header = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| anyhow!("Authorization header is missing"))?;

    let auth_header = header
        .to_str()
        .map_err(|_| anyhow!("Authorization header contains invalid characters"))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(anyhow!("Authorization scheme is not Bearer"));
    }

    Ok(&auth_header[7..])
}

async fn get_decoding_key(
    kid: &str,
    region: &str,
    user_pool_id: &str,
) -> Result<DecodingKey, anyhow::Error> {
    if let Some(key) = JWKS_CACHE.read().unwrap().get(kid) {
        return Ok(key.clone());
    }

    let jwks_url =
        format!("https://cognito-idp.{region}.amazonaws.com/{user_pool_id}/.well-known/jwks.json");

    let jwk_set: JwkSet = reqwest::Client::new()
        .get(&jwks_url)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to fetch JWKs: {}", e))?
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse JWKs: {}", e))?;

    let jwk = jwk_set
        .keys
        .into_iter()
        .find(|k| k.kid == kid)
        .ok_or_else(|| anyhow!("No JWK found for kid: {}", kid))?;

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|e| anyhow!("Failed to create decoding key: {}", e))?;

    JWKS_CACHE
        .write()
        .unwrap()
        .insert(kid.to_string(), decoding_key.clone());

    Ok(decoding_key)
}

fn create_unauthorized_response() -> HttpResponse {
    let error_response = ErrorResponse {
        message: "Unauthorized access".to_string(),
    };

    HttpResponse::Unauthorized()
        .content_type("application/json")
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header((
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ))
        .insert_header((
            "Access-Control-Allow-Headers",
            "Authorization, Content-Type",
        ))
        .json(error_response)
}

pub async fn reject_unauthorized_users(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    if req.method() == Method::OPTIONS {
        return next.call(req).await;
    }
    let cognito_settings = req
        .app_data::<web::Data<CognitoSettings>>()
        .ok_or_else(|| {
            InternalError::from_response(
                anyhow!("Cognito configuration not found in app data"),
                HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .json(ErrorResponse {
                        message: "Internal server error".to_string(),
                    }),
            )
        })?;

    let region = &cognito_settings.region;
    let user_pool_id = &cognito_settings.user_pool_id;
    let client_id = &cognito_settings.user_pool_client_id;

    let token = match extract_token_from_header(req.headers()) {
        Ok(token) => token,
        Err(e) => {
            // Log the specific error internally but return generic response
            let error_msg = format!("Authentication failed: {e}");
            return Err(InternalError::from_response(
                anyhow!(error_msg),
                create_unauthorized_response(),
            )
            .into());
        }
    };

    let token_header = match decode_header(token) {
        Ok(header) => header,
        Err(e) => {
            let error_msg = format!("Failed to decode token header: {e}");
            return Err(InternalError::from_response(
                anyhow!(error_msg),
                create_unauthorized_response(),
            )
            .into());
        }
    };

    let kid = match token_header.kid {
        Some(kid) => kid,
        None => {
            return Err(InternalError::from_response(
                anyhow!("No 'kid' found in token header"),
                create_unauthorized_response(),
            )
            .into())
        }
    };

    let decoding_key = match get_decoding_key(&kid, region, user_pool_id).await {
        Ok(key) => key,
        Err(e) => {
            return Err(InternalError::from_response(e, create_unauthorized_response()).into())
        }
    };

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[client_id.clone()]);
    validation.set_issuer(&[format!(
        "https://cognito-idp.{region}.amazonaws.com/{user_pool_id}"
    )]);

    let token_data = match decode::<CognitoClaims>(token, &decoding_key, &validation) {
        Ok(data) => data,
        Err(e) => {
            return Err(InternalError::from_response(
                anyhow!("Invalid token: {}", e),
                create_unauthorized_response(),
            )
            .into())
        }
    };

    if token_data.claims.token_use != "id" {
        return Err(InternalError::from_response(
            anyhow!("Token is not an ID token"),
            create_unauthorized_response(),
        )
        .into());
    }

    let user_id = match Uuid::parse_str(&token_data.claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return Err(InternalError::from_response(
                anyhow!("Invalid user ID in token"),
                create_unauthorized_response(),
            )
            .into())
        }
    };

    req.extensions_mut().insert(UserId(user_id));
    next.call(req).await
}
