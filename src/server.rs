// Server maintenance and check code here.
use console::style;
use indicatif::ProgressBar;
use std::time::Duration;

use crate::{
    db::{get_db_conn_pool, ping_db, redis_load, setup_db},
    mem_kv::{get_redis_pool, offload_all_cached_form_inputs, ping},
};

async fn check_network() {
    //Checks connection with Redis
    println!("{}", style("Performing Network Checks ").bold().cyan());
    {
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(100));
        bar.set_message("Connecting to Redis..");
        let redis_pool = get_redis_pool().await;
        ping(&redis_pool).await;
        let status = format!("Redis Database status        : {}", style("Active").green());
        bar.set_message(status);
        bar.finish();
    }
    //Checks connection with Postgres
    {
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(100));
        bar.set_message("Connecting to Postgres..");
        let postgres_pool = get_db_conn_pool().await;
        ping_db(&postgres_pool).await;
        let status = format!("Postgres Database status     : {}", style("Active").green());
        bar.set_message(status);
        bar.finish();
    }
}

pub async fn initialize() {
    println!("====================================================================");
    println!("{}", style("Starting Axum Super forms Server.").bold());
    println!(
        "{}",
        style("https://github.com/Mahinkumar/axum_super_forms").dim()
    );
    println!("--------------------------------------------------------------------");
    check_network().await;

    println!("{}", style("Initializing Redis Cache").bold().cyan());
    let (redis_pool, postgres_pool) = (get_redis_pool().await, get_db_conn_pool().await);
    setup_db(&postgres_pool).await;
    redis_load(&postgres_pool, &redis_pool).await;
}

pub async fn shutdown_commits() {
    let (redis_pool, postgres_pool) = (get_redis_pool().await, get_db_conn_pool().await);
    offload_all_cached_form_inputs(&redis_pool, &postgres_pool).await;
}
