use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateAlertRequest {
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

#[derive(Serialize, Debug)]
pub struct CreateAlertResponse {
    pub status: String,
}