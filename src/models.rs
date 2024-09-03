use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::axum_form_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AxumFormUser {
    pub uid: i32,
    pub name: String,
    pub passkey: String,
}