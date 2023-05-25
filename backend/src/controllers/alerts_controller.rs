use actix_web::{get, post, web, App, HttpServer, Result, Responder, HttpResponse, cookie};
use chrono::Utc;
use crate::controllers::alerts_dtos::{CreateAlertRequest, CreateAlertResponse};
use crate::domains::alert::Alert;
use crate::repository;
use crate::repository::database::Database;
use crate::services::alert_service::AlertService;


#[post("/alerts")]
pub async fn create_alert<>(service: web::Data<AlertService>,
                            db: web::Data<Database>,
                            request_payload: web::Json<CreateAlertRequest>) -> impl Responder {

    let create_alert_request_dto = request_payload.into_inner();
    //
    // // println!("cccc {:?}", &claims);

    let new_alert = Alert {
        id: Option::None,
        source: create_alert_request_dto.source,
        source_component: create_alert_request_dto.source_component,
        description: Option::Some(create_alert_request_dto.alert_description),
        alert_type: create_alert_request_dto.alert_type,
        subject_type: create_alert_request_dto.alert_subject_type,
        subject_reference_number: create_alert_request_dto.alert_subject_reference_number,
        subject_description: Option::Some(create_alert_request_dto.alert_subject_description),
        content: create_alert_request_dto.alert_content,
        created_at: Option::Some(Utc::now().naive_utc()),
        updated_at: Option::Some(Utc::now().naive_utc())
    };

    let todo = service.create_alert(&db, new_alert);
    // match todo {
    //     Ok(todo) => HttpResponse::Ok().json(todo),
    //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }

    HttpResponse::Ok().json("")
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



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_alert)
            // .service(get_todo_by_id)
            // .service(get_todos)
            // .service(delete_todo_by_id)
            // .service(update_todo_by_id)
    );
}