use futures_util::FutureExt;
use turreta_rust_keycloak::abra::keycloak_commons::{KeycloakOpenIdConnectClientContext, OpenIdAuthenticateResponse, OpenIdIssuerResponse, OpenIdUserInfoResponse};
use reqwest::Error;
use turreta_rust_keycloak::abra;

pub struct KeyCloakService {
    pub context: KeycloakOpenIdConnectClientContext
}

impl KeyCloakService {

    pub fn new() -> Self {

        println!("In keycloak service");

        KeyCloakService {
            context: KeycloakOpenIdConnectClientContext::new(
                "http://localhost:8080/auth".to_string(),
                "turreta-alerts".parse().unwrap(),
                "turreta-alerts-confidential-client".to_string(),
                "UqhfnkgfzWqdgUsJNqZqdUAXF3EJGpTu".to_string(),
                Option::None
            )
        }
    }

    pub async fn  authentication_and_get_token(username: &str, password: &str, context: &KeycloakOpenIdConnectClientContext) -> Result<OpenIdAuthenticateResponse, Error> {
        println!("DSDSDS");
        let auth_token = abra::keycloak_openid_service::KeycloakOpenIdConnectService::authenticate(
            username,
            password,
            context);



        let result = auth_token.await;
        result
    }

    pub async fn get_issue_details(context: &KeycloakOpenIdConnectClientContext) -> Result<OpenIdIssuerResponse, Error> {

        abra::keycloak_openid_service::KeycloakOpenIdConnectService::get_issuer_details(
            context).await
    }

    pub async fn get_issue_details2() -> String {

        "".to_owned()

    }
}