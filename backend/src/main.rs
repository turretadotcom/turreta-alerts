mod controllers;
mod repository;
mod domains;
mod services;
mod middlewares;

use actix_web::{get, post, web, App, HttpServer, Result, Responder, HttpResponse, cookie, middleware};
use serde::{Deserialize, Serialize};
use std::env;

use std::future::{ready, Ready};
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
use turreta_rust_keycloak::abra;
use turreta_rust_keycloak::abra::keycloak_commons::KeycloakOpenIdConnectClientContext;
use crate::controllers::alerts_controller::{config, create_alert};
use crate::middlewares::preprocessor::KeycloakPreAuth;
use crate::services::keycloak_service::KeyCloakService;

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

    let alerts_db = repository::database::Database::new();
    let context = KeycloakOpenIdConnectClientContext::new("turreta-alerts".parse().unwrap(),
                                                          "turreta-alerts-confidential-client".to_string(),
                                                          "UqhfnkgfzWqdgUsJNqZqdUAXF3EJGpTu".to_string());
    let keycloak_service = services::keycloak_service::KeyCloakService::new();

    // We try to retrieve the public key
    let string = KeyCloakService::get_issue_details("http://localhost:8080/auth/", &context).await;


    println!("String {}", string.unwrap().realm);


    // let app_data = web::Data::new(alerts_db);
    //
    // let alerts_service = services::alert_service::AlertService::new();
    // let app_alerts_service = web::Data::new(alerts_service);
    //
    // let keycloak_service = services::keycloak_service::KeyCloakService::new();
    // let app_keycloak_service = web::Data::new(keycloak_service);
    //
    // HttpServer::new(move ||
    //     App::new()
    //         .wrap(KeycloakPreAuth)
    //         .app_data(app_data.clone())
    //         .app_data(app_alerts_service.clone())
    //         .app_data(app_keycloak_service.clone())
    //         .configure(config)
    //         .service(healthcheck)
    //         .default_service(web::route().to(not_found))
    //         .wrap(actix_web::middleware::Logger::default())
    // )
    //     .bind(("127.0.0.1", 8081))?
    //     .run()
    //     .await

    std::env::set_var("RUST_LOG", "info,actix_web_middleware_keycloak_auth=trace");
    env_logger::init();

    HttpServer::new(|| {
        let keycloak_auth = KeycloakAuth::default_with_pk(
            DecodingKey::from_rsa_pem(KEYCLOAK_PK.as_bytes()).unwrap(),
        );

        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/private")
                    .wrap(keycloak_auth)
                    .route("", web::get().to(private)),
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