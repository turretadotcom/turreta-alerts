use actix_web::{get, post, web, App, HttpServer, Result, Responder, HttpResponse, cookie};
use crate::controllers::alerts_dtos::{CreateAlertRequest, CreateAlertResponse};


#[post("/alerts")]
pub async fn create_alert(request_payload: web::Json<CreateAlertRequest>) -> impl Responder {

    let response = &CreateAlertResponse {
        status: "Create alert".to_string(),
    };

    HttpResponse::Ok().json(response)
}


#[get("/alerts/{uuid}")]
async fn get_alert(uuid: web::Path<String>) -> &'static str {
    // let token_request = turreta_rust_keycloak::serde_json::json!({
    //     "grant_type":"password".to_string(),
    //     "username":"alerts".to_string(),
    //     "password":"password".to_string(),
    //     "client_id":"turreta-alerts-app".to_string(),
    //     "client_secret":"hk2IREWspYL3ALJApKQx0X2Q2qCd0fIw".to_string()
    //     // "redirect_uri":"".to_string(),
    //     // "code":"".to_string()}
    // });
    //
    // // let p = turreta_rust_keycloak::abra::openid::KeycloakClientContext {
    // //     urls: Urls {},
    // // };
    // //
    // // let tok = turreta_rust_keycloak::abra::keycloak::KeycloakOpenIdConnect::token(
    // //     "http://localhost:8080/auth/",token_request,"turreta-alerts");
    // // let o = tok.await ;
    // //  println!("{:?}", o);
    //
    // // "find alert by id"
    //
    // let ooo = &KeycloakClientContext::new(
    //     "turreta-alerts",
    //     "turreta-alerts-app".parse().unwrap(),
    //     "hk2IREWspYL3ALJApKQx0X2Q2qCd0fIw".parse().unwrap()
    // );
    // println!("{:?}", ooo);
    // let well = turreta_rust_keycloak::abra::keycloak::KeycloakOpenIdConnect::well_known("http://localhost:8080/auth/", ooo);
    // let o = well.await ;
    // println!("{}", o.unwrap());



    // OKOK
    // let well = turreta_rust_keycloak::keycloak::OpenId::well_known("http://localhost:8080/auth/","turreta-alerts");
    // let o = well.await ;
    // println!("{}", o.unwrap());

    // println!("{:?}",o.un);

    "find alert by id"
}
