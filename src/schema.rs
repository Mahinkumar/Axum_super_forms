// @generated automatically by Diesel CLI.

diesel::table! {
    axum_form_user (uid) {
        uid -> Int4,
        name -> Varchar,
        passkey -> Varchar,
    }
}
