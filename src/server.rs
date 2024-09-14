// Server maintenance and check code here.

use crate::{
    db::{get_db_conn_pool, ping_db, redis_load, setup_db},
    mem_kv::{get_redis_pool, offload_all_cached_form_inputs, ping},
};

async fn check_network() {
    let (redis_pool, postgres_pool) = (get_redis_pool().await, get_db_conn_pool().await);
    println!(
        "Redis Server Status          : {}",
        if ping(&redis_pool).await {
            "Active"
        } else {
            "Unable to connect"
        }
    );
    println!(
        "Postgres Server Status       : {}",
        if ping_db(&postgres_pool).await {
            "Active"
        } else {
            "Unable to connect"
        }
    );
}

pub async fn initialize() {
    let (redis_pool, postgres_pool) = (get_redis_pool().await, get_db_conn_pool().await);
    println!("=================================================================");
    println!("Starting Axum Super forms Server.");
    check_network().await;
    setup_db(&postgres_pool).await;
    redis_load(&postgres_pool, &redis_pool).await;
}

pub async fn shutdown_commits() {
    let (redis_pool, postgres_pool) = (get_redis_pool().await, get_db_conn_pool().await);
    offload_all_cached_form_inputs(&redis_pool, &postgres_pool).await;
    println!("Offloaded all cached form inputs to database.");
}
