use dotenv::dotenv;
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

pub async fn setup_db(){
    let conn = get_db_conn_pool().await;
    sqlx::query_file!("sql/setup_admin.sql").execute(&conn).await.expect("Unable to setup Admin db!");
    sqlx::query_file!("sql/setup_user.sql").execute(&conn).await.expect("Unable to setup User db!");
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
