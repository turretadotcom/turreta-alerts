use serde::{Deserialize, Serialize};
// use validator::Validate;

/// Web layer DTO for creating alerts
///
#[derive(Deserialize,Serialize, Debug, Default)]
// #[derive(Default,Debug,Queryable,Identifiable, Insertable,Serialize,Deserialize)]
pub struct CreateAlertRequest {
    // id: String,
    // client_id: String,
    // #[validate(length(min = 3, max = 50))]
    pub source: String,

    // #[validate(length(min = 3, max = 50))]
    pub source_component: String,

    // #[validate(length(min = 3, max = 50))]
    pub alert_type: String,

    // #[validate(length(max = 200))]
    pub alert_description: String,

    // #[validate(length(min = 3, max = 50))]
    pub alert_subject_type: String,

    // #[validate(length(min = 3, max = 50))]
    pub alert_subject_reference_number: String,

    // #[validate(length(max = 50))]
    pub alert_subject_description: String,

    // #[validate(length(min=3, max = 65535))]
    pub alert_content: String,
}

#[derive(Serialize, Debug)]
pub struct CreateAlertResponse {
    pub status: String,
}