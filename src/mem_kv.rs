
use bb8_redis::bb8;
use bb8_redis::bb8::Pool;
use bb8_redis::redis::AsyncCommands;
use bb8_redis::RedisConnectionManager;
//use bb8::{Pool, PooledConnection};
use std::time::{SystemTime, UNIX_EPOCH};

use dotenvy::dotenv;
use sqlx::Postgres;
use std::env;

use crate::{
    db::{FormData, User},
    forms::FormInputAll,
};

pub async fn get_redis_pool() -> Pool<RedisConnectionManager> {
    dotenv().ok();
    let manager = RedisConnectionManager::new(
        env::var("REDIS_CONNECTION_URL").expect("env variable REDIS_CONNECTION_URL must be set!"),
    )
    .unwrap();

    bb8::Pool::builder().build(manager).await.unwrap()
}

pub async fn ping(conn_pool: &Pool<RedisConnectionManager>) -> bool {
    let mut conn = conn_pool.get().await.unwrap();
    conn.set::<&str, &str, ()>("Check", "Response recieved!")
        .await
        .unwrap();
    conn.get::<&str, String>("Check").await.unwrap() == *"Response recieved!"
}

pub async fn retrieve_user_redis(
    key: String,
    conn_pool: &Pool<RedisConnectionManager>,
) -> Result<User, bb8_redis::redis::RedisError> {
    let key = format!("{key}_Userkey");
    let mut redis_conn = conn_pool.get().await.unwrap();

    redis_conn.get::<&str, User>(&key).await
}

pub async fn retrieve_forms(
    key: &String,
    conn_pool: &Pool<RedisConnectionManager>,
) -> Result<FormData, bb8_redis::redis::RedisError> {
    let key = format!("{key}_Formkey");
    let mut redis_conn = conn_pool.get().await.expect("Unable to acquire connection");

    redis_conn.get::<&str, FormData>(&key).await
}

pub async fn cache_form_input(
    user_id: &String,
    form: &String,
    conn_pool: &Pool<RedisConnectionManager>,
    inputs: FormInputAll,
) {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().to_string();
    let key = format!("{user_id}_{form}_{time}_FormIK");
    let mut redis_conn = conn_pool.get().await.expect("Unable to acquire connection");
    redis_conn
        .set::<&str, &FormInputAll, ()>(&key, &inputs)
        .await
        .unwrap();

    //for test only
    //offload_all_cached_form_inputs(&conn_pool, &get_db_conn_pool().await).await;
}


// Offload all stored forms from redis cache to database.
pub async fn offload_all_cached_form_inputs(
    conn_pool: &Pool<RedisConnectionManager>,
    db_conn_pool: &sqlx::Pool<Postgres>
){
    let mut redis_conn = conn_pool.get().await.expect("Unable to acquire connection");
    let keys: Vec<String> = redis_conn.keys("*_FormIK").await.unwrap();
    for key in keys {
        let cached_input: FormInputAll = redis_conn.get(&key).await.unwrap();
        for vals in cached_input.inputs{
            let uid: i32 = cached_input.user_id.parse().expect("Unable to parse user id. USER ID NOT AN INTEGER");
            sqlx::query("INSERT INTO form_data(username,user_id,fid,input_name,input_value) VALUES($1,$2,$3,$4,$5) ON CONFLICT DO NOTHING;")
                .bind(&cached_input.uname)
                .bind(&uid)
                .bind(&cached_input.fname)
                .bind(vals.name)
                .bind(vals.value)
                .execute(db_conn_pool)
                .await
                .expect("Unable to create DEFAULT form in forms table");
        }
        redis_conn.del::<&str,()>(&key).await.expect("Unable to clear key after offloading to db");
    }
}

