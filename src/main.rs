use axum::extract::Request;
use axum::Router;
use axum::ServiceExt;
use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;
use db::get_db_conn_pool;
use db::redis_load;
use db::setup_db;
use forms::form_router;
use mem_kv::get_redis_pool;
use sqlx::Postgres;
use tower::Layer;
use tower_http::normalize_path::NormalizePath;
use tower_http::normalize_path::NormalizePathLayer;
use std::net::SocketAddr;

pub mod admin;
pub mod auth;
pub mod client;
pub mod db;
pub mod forms;
pub mod jwt_auth;
pub mod mem_kv;
pub mod router;

use admin::admin_router;
use client::client_router;
use db::ping_db;
use mem_kv::ping;
use router::general_router;
use router::login_router;

//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
//use tower_http::trace::TraceLayer;
#[derive(Clone)]
pub struct DbPools {
    pub postgres_pool: sqlx::Pool<Postgres>,
    pub redis_pool: Pool<RedisConnectionManager>,
}

const ADDR: [u8; 4] = [127, 0, 0, 1];
const PORT_HOST: u16 = 8000;

#[tokio::main]

async fn main() {
    // Use for Debug only!! Heavily reduces perfomance
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
    //             format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
    //         }),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    let redis_pool = get_redis_pool().await;
    let postgres_pool = get_db_conn_pool().await;

    let database_pools = DbPools {
        postgres_pool: postgres_pool.clone(),
        redis_pool: redis_pool.clone(),
    };

    println!("=================================================================");
    println!("Starting Axum Super forms Server.");
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

    setup_db(&postgres_pool).await;
    redis_load(&postgres_pool, &redis_pool).await;
    let axum_router = Router::new()
        .merge(login_router())
        .merge(admin_router())
        .merge(general_router())
        .merge(client_router())
        .merge(form_router());

    let app: Router = axum_router.with_state(database_pools);
    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    tokio::join!(serve(app, PORT_HOST));
}

async fn serve(app: NormalizePath<Router>, port: u16) {
    println!("Serving on address           : http://127.0.0.1:{port}");
    println!("=================================================================");
    let addr = SocketAddr::from((ADDR, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,  ServiceExt::<Request>::into_make_service(app)).await.unwrap();
}

