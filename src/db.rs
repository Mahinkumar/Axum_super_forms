use crate::auth::hash_password;
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::env;

pub async fn get_db_conn_pool() -> sqlx::Pool<Postgres> {
    dotenv().ok();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("env variable DATABASE_URL must be set!"))
        .await
        .expect("Unable to create a connection pool")
}

pub async fn setup_db() {
    let conn = get_db_conn_pool().await;
    sqlx::query_file!("sql/setup_admin.sql")
        .execute(&conn)
        .await
        .expect("Unable to setup Admin Table!");

    let admin_email =
        &env::var("DEFAULT_ADMIN_MAIL").expect("env variable DEFAULT_ADMIN_MAIL must be set!");

    let admin_passkey =
        &env::var("DEFAULT_ADMIN_KEY").expect("env variable DEFAULT_ADMIN_KEY must be set!");

    let admin_hash = hash_password(
        &env::var("DEFAULT_ADMIN_PASSWORD")
            .expect("env variable DEFAULT_ADMIN_PASSWORD must be set!")
            .as_bytes(),
    )
    .await;

    sqlx::query("INSERT INTO  admins(aid,email,username,passkey,passhash) VALUES(0,$1,'ADMIN',$2,$3) ON CONFLICT DO NOTHING;")
        .bind(admin_email)
        .bind(admin_passkey)
        .bind(admin_hash)
        .execute(&conn)
        .await
        .expect("Unable to create DEFAULT ADMIN");

    sqlx::query_file!("sql/setup_user.sql")
        .execute(&conn)
        .await
        .expect("Unable to setup User Table!");
}

pub async fn ping_db() -> bool {
    let pool = get_db_conn_pool().await;
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .expect("Unable to make test query");
    row.0 == 150
}

pub async fn retrieve_admin(e_mail: String)-> Result<(i32,String,String),sqlx::Error>{
    let pool = get_db_conn_pool().await;
    sqlx::query_as("SELECT aid,username,passhash FROM admins WHERE email=$1").bind(e_mail).fetch_one(&pool).await
}
