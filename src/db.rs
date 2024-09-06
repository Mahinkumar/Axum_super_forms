use crate::auth::hash_password;
use redis::AsyncCommands;
use bb8_redis::{bb8::Pool, redis, RedisConnectionManager};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::env;
use redis_macros::{FromRedisValue, ToRedisArgs};

//userid,email,username,passkey
#[derive(Debug)]
#[derive(Deserialize, FromRedisValue)]
#[derive(Serialize, ToRedisArgs)]
pub struct User {
    pub userid: i32,
    pub email: String,
    pub username: String,
    pub passkey: String,
}

pub async fn get_db_conn_pool() -> sqlx::Pool<Postgres> {
    dotenv().ok();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("env variable DATABASE_URL must be set!"))
        .await
        .expect("Unable to create a connection pool")
}

pub async fn setup_db(conn: &sqlx::Pool<Postgres>) {

    sqlx::query_file!("sql/setup_admin.sql")
        .execute(conn)
        .await
        .expect("Unable to setup Admin Table!");

    sqlx::query_file!("sql/setup_user.sql")
        .execute(conn)
        .await
        .expect("Unable to setup User Table!");

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

    sqlx::query("INSERT INTO admins(aid,email,username,passhash) VALUES(0,$1,'ADMIN',$2) ON CONFLICT DO NOTHING;")
        .bind(admin_email)
        .bind(admin_hash)
        .execute(conn)
        .await
        .expect("Unable to create DEFAULT ADMIN in admins table");

    sqlx::query("INSERT INTO forms_user(userid,email,username,passkey) VALUES(0,$1,'ADMIN',$2) ON CONFLICT DO NOTHING;")
        .bind(admin_email)
        .bind(admin_passkey)
        .execute(conn)
        .await
        .expect("Unable to create DEFAULT ADMIN in forms_user table");
}

pub async fn ping_db(conn: &sqlx::Pool<Postgres>) -> bool {
    let row: (i64,) = sqlx::query_as("SELECT $1;")
        .bind(150_i64)
        .fetch_one(conn)
        .await
        .expect("Unable to make test query");
    row.0 == 150
}

pub async fn retrieve_admin(conn: sqlx::Pool<Postgres>,e_mail: String) -> Result<(i32, String, String), sqlx::Error> {
    sqlx::query_as("SELECT aid,username,passhash FROM admins WHERE email=$1 ;")
        .bind(e_mail)
        .fetch_one(&conn)
        .await
}

pub async fn retrieve_user(conn: &sqlx::Pool<Postgres>,key: String) -> Result<(i32, String, String), sqlx::Error> {
    sqlx::query_as("SELECT userid,username,email FROM forms_user WHERE passkey=$1 ;")
        .bind(key)
        .fetch_one(conn)
        .await
}

pub async fn redis_copy(conn: &sqlx::Pool<Postgres>,redis_pool: &Pool<RedisConnectionManager>){
    let mut redis_conn = redis_pool.get().await.unwrap();
    let all_kv: Vec<(i32,String, String, String)> =  sqlx::query_as("SELECT * FROM forms_user;").fetch_all(conn).await.expect("Unable to fetch from Database");
    print!("Setting up Redis          : ");
    let mut count = 0;
    for i in all_kv{
        count += 1;
        let j = User {email: i.1, username:i.2, passkey:i.3.clone(), userid:i.0};
        redis_conn.set::<&str,&User, ()>(&i.3,&j).await.expect("Unable to load db into memory");
    }
    println!("Loaded {count} entries into Memory.");
}

