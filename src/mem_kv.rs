use bb8_redis::bb8;
use bb8_redis::bb8::Pool;
use bb8_redis::redis::AsyncCommands;
use bb8_redis::RedisConnectionManager;
//use bb8::{Pool, PooledConnection};

use dotenvy::dotenv;
use std::env;

use crate::db::User;

pub async fn get_redis_pool() -> Pool<RedisConnectionManager> {
    dotenv().ok();
    let manager = RedisConnectionManager::new(
        env::var("REDIS_CONNECTION_URL").expect("env variable REDIS_CONNECTION_URL must be set!"),
    )
    .unwrap();

    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    pool
}

pub async fn ping(conn_pool: &Pool<RedisConnectionManager>) -> bool {
    let mut conn = conn_pool.get().await.unwrap();
    conn.set::<&str, &str, ()>("Check", "Response recieved!")
        .await
        .unwrap();
    conn.get::<&str, String>("Check").await.unwrap() == "Response recieved!".to_string()
}

pub async fn retrieve_user_redis(key: String,conn_pool: &Pool<RedisConnectionManager>)->Result<User, bb8_redis::redis::RedisError>{
    let mut redis_conn = conn_pool.get().await.unwrap();
    let value = redis_conn.get::<&str, User>(&key).await;
    value
}