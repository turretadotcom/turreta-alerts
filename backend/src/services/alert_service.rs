use crate::domains::alert::Alert;
use crate::repository::database::Database;
use std::fmt::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use crate::repository::schema::alerts::dsl::alerts;

pub struct AlertService {

}

impl AlertService {

    pub fn new() -> Self {
        AlertService {}
    }

    pub fn create_alert(&self, db_ref: &Database, alert: Alert) -> Result<Alert, Error> {
        let new_alert = Alert {
            id: Option::None,
            source: alert.source,
            source_component: alert.source_component,
            description: Option::None,
            alert_type: alert.alert_type,
            subject_type: alert.subject_type,
            subject_reference_number: alert.subject_reference_number,
            subject_description: Option::Some(String::from("Subject description")),
            content: alert.content,
            created_at: Option::Some(Utc::now().naive_utc()),
            updated_at: Option::Some(Utc::now().naive_utc()),
        };
        diesel::insert_into(alerts)
            .values(&new_alert)
            .execute(&mut db_ref.pool.get().unwrap())
            .expect("Error creating new alert");
        Ok(new_alert)
    }

}

