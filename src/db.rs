use crate::schema::axum_form_user::dsl::*;
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use dotenv::dotenv;
use std::env;

pub async fn get_db_conn() -> AsyncPgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn ping_db()->bool{
    let mut test_conn = get_db_conn().await;

    diesel::insert_into(axum_form_user)
        .values((name.eq("Axum_super_forms"), uid.eq(0), passkey.eq("00000000")))
        .execute(&mut test_conn)
        .await
        .expect("Unable to Perform test Insert");

    diesel::delete(axum_form_user.filter(uid.eq(0)))
        .execute(&mut test_conn)
        .await
        .expect("Unable to Delete test transaction");

    return true;
}
