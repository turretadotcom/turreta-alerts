#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::http::RawStr;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateAlertRequest {
    // id: String,
    // client_id: String,
    source: String,
    source_component: String,
    alert_type: String,
    alert_description: String,
    alert_subject_type: String,
    alert_subject_reference_number: String,
    alert_subject_description: String,
    alert_content: String,
    created_at: String,
    updated_at: String
}


#[post("/alerts", format = "json", data="<request>")]
fn create_alert(request: Json<CreateAlertRequest>) -> &'static str {
    "Create alert"
}

#[get("/alerts")]
fn get_recent_alert() -> &'static str {
    "get recent alerts via default date range"
}

#[get("/alerts/<uuid>")]
fn get_alert(uuid: String) -> &'static str {
    "find alert by id"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![create_alert, get_alert, get_recent_alert])
}