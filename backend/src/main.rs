mod controllers;
mod repository;
mod domains;
mod services;

use actix_web::{get, post, web, App, HttpServer, Result, Responder, HttpResponse, cookie};
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
use turreta_rust_keycloak::abra;
use turreta_rust_keycloak::abra::keycloak_commons::KeycloakOpenIdConnectClientContext;
use crate::controllers::alerts_controller::{config, create_alert};

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

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    let alerts_db = repository::database::Database::new();
    let app_data = web::Data::new(alerts_db);

    let alerts_service = services::alert_service::AlertService::new();
    let app_alerts_service = web::Data::new(alerts_service);


    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .app_data(app_alerts_service.clone())
            .configure(config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}