use axum::{extract::Request, Router, ServiceExt};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use server::shutdown_commits;
use sqlx::Postgres;
use std::net::SocketAddr;
use tokio::signal;
use tokio::time::Duration;
use tower::Layer;
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};
use tower_http::timeout::TimeoutLayer;

pub mod admin;
pub mod auth;
pub mod client;
pub mod db;
pub mod forms;
pub mod jwt_auth;
pub mod mem_kv;
pub mod router;
pub mod server;

use admin::admin_router;
use client::client_router;
use db::get_db_conn_pool;
use forms::form_router;
use mem_kv::get_redis_pool;
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
    let (redis_pool, postgres_pool) = tokio::join!(get_redis_pool(), get_db_conn_pool());

    let database_pools = DbPools {
        postgres_pool,
        redis_pool,
    };

    crate::server::initialize().await;

    let axum_router = Router::new()
        .merge(login_router())
        .merge(admin_router())
        .merge(general_router())
        .merge(client_router())
        .merge(form_router())
        .layer(TimeoutLayer::new(Duration::from_secs(5)));

    let app: Router = axum_router.with_state(database_pools);
    let app = NormalizePathLayer::trim_trailing_slash().layer(app);
    tokio::spawn(async move { serve(app, PORT_HOST).await })
        .await
        .expect("Unable to Spawn Threads")
}

async fn serve(app: NormalizePath<Router>, port: u16) {
    println!("Serving on address           : http://127.0.0.1:{port}");
    println!("=================================================================");
    let addr = SocketAddr::from((ADDR, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        graceful_shutdown_procedure().await
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
        graceful_shutdown_procedure().await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn graceful_shutdown_procedure() {
    println!();
    println!("Shutdown Initiated");

    shutdown_commits().await;
    // We offload redis data to db here
    println!("Performing a graceful shutdown");
}
