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
                "turreta-alerts".parse().unwrap(),
                "turreta-alerts-confidential-client".to_string(),
                "UqhfnkgfzWqdgUsJNqZqdUAXF3EJGpTu".to_string()
            )
        }
    }

    pub async fn  authentication_and_get_token(base_url: &str, username: &str, password: &str, context: &KeycloakOpenIdConnectClientContext) -> Result<OpenIdAuthenticateResponse, Error> {
        println!("DSDSDS");
        let auth_token = abra::keycloak_openid_service::KeycloakOpenIdConnectService::authenticate(
            base_url,
            username,
            password,
            context);



        let result = auth_token.await;
        result
    }

    pub async fn get_issue_details(base_url: &str, context: &KeycloakOpenIdConnectClientContext) -> Result<OpenIdIssuerResponse, Error> {

        abra::keycloak_openid_service::KeycloakOpenIdConnectService::get_issuer_details(
            base_url,
            context).await
    }

    pub async fn get_issue_details2() -> String {

        "".to_owned()

    }
}