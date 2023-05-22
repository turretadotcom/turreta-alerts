use std::future::{Future, ready, Ready};
use std::rc::Rc;

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, web};
use actix_web::error::ErrorUnauthorized;
use futures_util::future::LocalBoxFuture;
use futures_util::{FutureExt, TryFutureExt};
use turreta_rust_keycloak::abra::keycloak_commons::{KeycloakOpenIdConnectClientContext, OpenIdAuthenticateResponse};
use crate::repository::database::Database;
use crate::services::keycloak_service::KeyCloakService;

pub struct KeycloakPreAuth;

impl KeycloakPreAuth {
    pub fn p() {
        println!("FDFFDF");
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for KeycloakPreAuth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = KeycloakPreAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(KeycloakPreAuthMiddleware { service }))
    }


}

pub struct KeycloakPreAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for KeycloakPreAuthMiddleware<S>
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

        let fut = self.service.call(req);

        let o = t();
        Box::pin(async move {

            o.await;
            //

            //
            // let result = KeyCloakService::authentication_and_get_token("http://localhost:8080/auth/",
            //                                                            "tauser1", "password", &key_cloak_service.context);


            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
            // Err(ErrorUnauthorized("not authorized"))
        })
    }


}

async fn t() {
    println!("ddd");
}