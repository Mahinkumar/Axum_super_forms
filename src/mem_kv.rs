use bb8_redis::bb8;
use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;
//use bb8::{Pool, PooledConnection};

use dotenv::dotenv;
use redis::AsyncCommands;
use std::env;


pub async fn get_redis_con() -> Pool<RedisConnectionManager>{
    dotenv().ok();
    let manager = RedisConnectionManager::new(
        env::var("REDIS_CONNECTION_URL").expect("env variable REDIS_CONNECTION_URL must be set!"),
    )
    .unwrap();

    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    pool
}

pub async fn ping() -> bool{
    let pool = get_redis_con().await;
    let mut conn = pool.get().await.unwrap();
    conn.set::<&str, &str, ()>("Check", "Response recieved!")
        .await
        .unwrap();
    conn.get::<&str, String>("Check").await.unwrap() == "Response recieved!".to_string()
} 
