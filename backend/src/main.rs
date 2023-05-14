mod controllers;


use dotenvy::{dotenv};
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

// use turreta_rust_keycloak::abra::keycloak::KeycloakClientContext;







// #[get("/alerts")]
// async fn get_recent_alert() -> &'static str {
//     "get recent alerts via default date range"
// }


#[derive(Default)]
struct TokenCheckerMiddleware {}
//
// #[rocket::async_trait]
// impl Fairing for TokenCheckerMiddleware {
//     fn info(&self) -> Info {
//         Info {
//             name: "GET/POST Counter",
//             kind: Kind::Request | Kind::Response
//         }
//     }
//     async fn on_request(&self, _req: &mut Request<'_>, _: &mut Data<'_>) {
//         let t = _req.headers().get_one("Authorization");
//
//         // println!("{}", env!("KEYCLOAK_CLIENT_ID"));
//         // println!("{}", env!("KEYCLOAK_CLIENT_SECRET"));
//
//         for (key, value) in env::vars() {
//             println!("{key}: {value}");
//         }
//
//         match t {
//             Some(token) => {
//                 println!("{}", token);
//             }
//             None => {
//                 println!("None");
//
//             }
//         }
//     }
// }


pub struct KeycloakTransformFactory;

impl KeycloakTransformFactory {
    pub fn new() -> Self {
        KeycloakTransformFactory {
            // auth_data: Rc::new(auth_data),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for KeycloakTransformFactory
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = KeycloakMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(KeycloakMiddleware { service }))
    }
}


pub struct KeycloakMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for KeycloakMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(KeycloakTransformFactory::new())
            // .wrap(SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
            //     .cookie_secure(false)
            //     // customize session and cookie expiration
            //     .session_lifecycle(
            //         PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
            //     ))
            .service( controllers::alerts_controller::create_alert)
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}