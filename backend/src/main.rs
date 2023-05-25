mod controllers;
mod repository;
mod domains;
mod services;
mod middlewares;
mod utils;

use actix_web::{get, post, web, App, HttpServer, Result, Responder, HttpResponse, cookie, middleware};
use serde::{Deserialize, Serialize};
use std::env;

use std::future::{ready, Ready};
use std::string::ToString;
use actix_session::config::PersistentSession;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use futures_util::future::LocalBoxFuture;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use actix_web::cookie::Key;
use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth, StandardKeycloakClaims};
use dotenv::dotenv;
use turreta_rust_keycloak::abra;
use turreta_rust_keycloak::abra::keycloak_commons::KeycloakOpenIdConnectClientContext;
use crate::controllers::alerts_controller::{config, create_alert};
use crate::middlewares::preprocessor::KeycloakPreAuth;
use crate::services::keycloak_service::KeyCloakService;
use crate::utils::common_utils::public_key_normalize_format;

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}




#[derive(Serialize)]
pub struct Response {
    pub message: String,
}
async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

const KEYCLOAK_PK: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAs0zgtMNI2KgHotIYaJMf
/kuBcK7SXuQuC2pYI9M/9Lpaodt5Q2VajIB8DgMykjhRri1E0319T0/qRqrZPdP7
36JkKMgpTn9vBnGw/KVRTQ3sfRiNZdZmlAyKXl5RgB+Y/c+cD4XP3yp6uv/apWOj
bkjvGYrWiRU4mcQrgEbYuLJ5vbmfjGeTOnw0+SXUgCt0O2r02yIFvjHZPrWyr5Ds
Z/w9BR4a6L4XPiptPrMuybK3wbZf6OEtefm2fzH5YCsg2R12F19XR0lYBLcB+syb
K4lYVHW8Jp/bVhAHTS2bmljUHJhpuP4V86nJABATPkmUnx6R6kSFVKyEKb+SZy9F
eQIDAQAB
-----END PUBLIC KEY-----";


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let keycloak_base_url = std::env::var("KEYCLOAK_BASE_URL").expect("KEYCLOAK_BASE_URL must be set");
    let keycloak_realm_name = std::env::var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set");
    let keycloak_client_id = std::env::var("KEYCLOAK_CLIENT_ID").expect("KEYCLOAK_CLIENT_ID must be set");
    let keycloak_client_secret = std::env::var("KEYCLOAK_CLIENT_SECRET").expect("KEYCLOAK_CLIENT_SECRET must be set");


    let alerts_db = repository::database::Database::new();
    let context = KeycloakOpenIdConnectClientContext::new(
        keycloak_base_url.clone(),
        keycloak_realm_name.clone(),
        keycloak_client_id.clone(),
        keycloak_client_secret.clone(),
        Option::None
    );

    // We try to retrieve the public key
    let string = KeyCloakService::get_issue_details(&context).await;


    // println!("String {}", string.unwrap().public_key);
    let pp = public_key_normalize_format(string.unwrap().public_key.clone());
    // println!("String {}", &pp);

    std::env::set_var("RUST_LOG", "info,actix_web=trace");
    env_logger::init();

    let app_data = web::Data::new(alerts_db);

    let alerts_service = services::alert_service::AlertService::new();
    let app_alerts_service = web::Data::new(alerts_service);

    let keycloak_service = services::keycloak_service::KeyCloakService::new();
    let app_keycloak_service = web::Data::new(keycloak_service);

    HttpServer::new(move || {
        let keycloak_auth = KeycloakAuth::default_with_pk(
            DecodingKey::from_rsa_pem(&pp.clone().to_owned().as_bytes()).unwrap(),
        );

        App::new()
            .wrap(middleware::Logger::default())
            .service(
            web::scope("/api")
                .wrap(keycloak_auth)
                        .app_data(app_data.clone())
                        .app_data(app_alerts_service.clone())
                        .app_data(app_keycloak_service.clone())
                .service(create_alert)
            )
            .service(web::resource("/").to(hello_world))
    })
        .bind("127.0.0.1:8081")?
        .workers(1)
        .run()
        .await
}

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn private(claims: StandardKeycloakClaims) -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", &claims))
}