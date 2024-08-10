// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        #[max_length = 350]
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        created_at -> Timestamp,
    }
}
