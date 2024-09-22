// Server maintenance and check code here.
use indicatif::ProgressBar;
use std::time::Duration;

use crate::{
    db::{get_db_conn_pool, ping_db, redis_load, setup_db},
    mem_kv::{get_redis_pool, offload_all_cached_form_inputs, ping},
};

async fn check_network() {
    //Checks connection with Redis
    println!("Performing Network Checks:");
    {
        
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(100));
        bar.set_message("Connecting to Redis..");
        let redis_pool = get_redis_pool().await;
        ping(&redis_pool).await;
        bar.set_message("=> Redis DB Active");
        bar.finish();
    }

    //Checks connection with Postgres
    {
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(100));
        bar.set_message("Connecting to Postgres..");
        let postgres_pool = get_db_conn_pool().await;
        ping_db(&postgres_pool).await;
        bar.set_message("=> Postgres DB Active");
        bar.finish();
    }
}

pub async fn initialize() {
    println!("=================================================================");
    println!("Starting Axum Super forms Server.");
    check_network().await;
    let (redis_pool, postgres_pool) = (get_redis_pool().await, get_db_conn_pool().await);
    setup_db(&postgres_pool).await;
    redis_load(&postgres_pool, &redis_pool).await;
}

pub async fn shutdown_commits() {
    let (redis_pool, postgres_pool) = (get_redis_pool().await, get_db_conn_pool().await);
    offload_all_cached_form_inputs(&redis_pool, &postgres_pool).await;
}
