// @generated automatically by Diesel CLI.

diesel::table! {
    alerts (id) {
        id -> Int8,
        source -> Varchar,
        source_component -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        description -> Nullable<Text>,
        subject_type -> Varchar,
        subject_reference_number -> Varchar,
        subject_description -> Nullable<Varchar>,
        content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
